// spell-checker:ignore dword, minwindef, ntdef, ntdll, ntstatus, osversioninfoex, osversioninfoexa
// spell-checker:ignore osversioninfoexw, serverr, sysinfoapi, winnt, winuser, pbool, libloaderapi
// spell-checker:ignore lpcstr, processthreadsapi, farproc, lstatus, wchar, lpbyte, hkey, winerror
// spell-checker:ignore osstr, winreg

#![allow(unsafe_code)]

use std::{
    ffi::{OsStr, OsString},
    mem,
    os::windows::ffi::{OsStrExt, OsStringExt},
    ptr,
};

use winapi::{
    shared::{
        minwindef::{DWORD, FARPROC, LPBYTE},
        ntdef::{LPCSTR, NTSTATUS},
        ntstatus::STATUS_SUCCESS,
        winerror::ERROR_SUCCESS,
    },
    um::{
        libloaderapi::{GetModuleHandleA, GetProcAddress},
        sysinfoapi::{GetSystemInfo, SYSTEM_INFO},
        winnt::{
            KEY_READ, PROCESSOR_ARCHITECTURE_AMD64, REG_SZ, VER_NT_WORKSTATION,
            VER_SUITE_WH_SERVER, WCHAR,
        },
        winreg::{RegOpenKeyExW, RegQueryValueExW, HKEY_LOCAL_MACHINE, LSTATUS},
        winuser::{GetSystemMetrics, SM_SERVERR2},
    },
};

use crate::{Bitness, Info, Type, Version};

#[cfg(target_arch = "x86")]
type OSVERSIONINFOEX = winapi::um::winnt::OSVERSIONINFOEXA;

#[cfg(not(target_arch = "x86"))]
type OSVERSIONINFOEX = winapi::um::winnt::OSVERSIONINFOEXW;

pub fn get() -> Info {
    let (version, edition) = version();
    Info {
        os_type: Type::Windows,
        version,
        edition,
        bitness: bitness(),
        ..Default::default()
    }
}

fn version() -> (Version, Option<String>) {
    match version_info() {
        None => (Version::Unknown, None),
        Some(v) => (
            Version::Semantic(
                v.dwMajorVersion as u64,
                v.dwMinorVersion as u64,
                v.dwBuildNumber as u64,
            ),
            product_name().or_else(|| edition(&v)),
        ),
    }
}

#[cfg(target_pointer_width = "64")]
fn bitness() -> Bitness {
    // x64 program can only run on x64 Windows.
    Bitness::X64
}

#[cfg(target_pointer_width = "32")]
fn bitness() -> Bitness {
    use winapi::{
        shared::{
            minwindef::{BOOL, FALSE, PBOOL},
            ntdef::HANDLE,
        },
        um::processthreadsapi::GetCurrentProcess,
    };

    // IsWow64Process is not available on all supported versions of Windows. Use GetModuleHandle to
    // get a handle to the DLL that contains the function and GetProcAddress to get a pointer to the
    // function if available.
    let is_wow_64 = match get_proc_address(b"kernel32\0", b"IsWow64Process\0") {
        None => return Bitness::Unknown,
        Some(val) => val,
    };

    type IsWow64 = unsafe extern "system" fn(HANDLE, PBOOL) -> BOOL;
    let is_wow_64: IsWow64 = unsafe { mem::transmute(is_wow_64) };

    let mut result = FALSE;
    if unsafe { is_wow_64(GetCurrentProcess(), &mut result) } == 0 {
        log::error!("IsWow64Process failed");
        return Bitness::Unknown;
    }

    if result == FALSE {
        Bitness::X32
    } else {
        Bitness::X64
    }
}

// Calls the Win32 API function RtlGetVersion to get the OS version information:
// https://msdn.microsoft.com/en-us/library/mt723418(v=vs.85).aspx
fn version_info() -> Option<OSVERSIONINFOEX> {
    let rtl_get_version = match get_proc_address(b"ntdll\0", b"RtlGetVersion\0") {
        None => return None,
        Some(val) => val,
    };

    type RtlGetVersion = unsafe extern "system" fn(&mut OSVERSIONINFOEX) -> NTSTATUS;
    let rtl_get_version: RtlGetVersion = unsafe { mem::transmute(rtl_get_version) };

    let mut info: OSVERSIONINFOEX = unsafe { mem::zeroed() };
    info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEX>() as DWORD;

    if unsafe { rtl_get_version(&mut info) } == STATUS_SUCCESS {
        Some(info)
    } else {
        None
    }
}

fn product_name() -> Option<String> {
    const REG_SUCCESS: LSTATUS = ERROR_SUCCESS as LSTATUS;

    let sub_key = to_wide("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
    let mut key = ptr::null_mut();
    if unsafe { RegOpenKeyExW(HKEY_LOCAL_MACHINE, sub_key.as_ptr(), 0, KEY_READ, &mut key) }
        != REG_SUCCESS
        || key.is_null()
    {
        log::error!("RegOpenKeyExW(HKEY_LOCAL_MACHINE, ...) failed");
        return None;
    }

    // Get size of the data.
    let name = to_wide("ProductName");
    let mut data_type: DWORD = 0;
    let mut data_size: DWORD = 0;
    if unsafe {
        RegQueryValueExW(
            key,
            name.as_ptr(),
            ptr::null_mut(),
            &mut data_type,
            ptr::null_mut(),
            &mut data_size,
        )
    } != REG_SUCCESS
        || data_type != REG_SZ
        || data_size == 0
        || data_size % 2 != 0
    {
        log::error!("RegQueryValueExW failed");
        return None;
    }

    // Get the data.
    let mut data = vec![0u16; data_size as usize / 2];
    if unsafe {
        RegQueryValueExW(
            key,
            name.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            data.as_mut_ptr() as LPBYTE,
            &mut data_size,
        )
    } != REG_SUCCESS
        || data_size as usize != data.len() * 2
    {
        return None;
    }

    // If the data has the REG_SZ, REG_MULTI_SZ or REG_EXPAND_SZ type, the string may not have been
    // stored with the proper terminating null characters.
    match data.last() {
        Some(0) => {
            data.pop();
        }
        _ => {}
    }

    Some(
        OsString::from_wide(data.as_slice())
            .to_string_lossy()
            .into_owned(),
    )
}

fn to_wide(value: &str) -> Vec<WCHAR> {
    OsStr::new(value).encode_wide().chain(Some(0)).collect()
}

// Examines data in the OSVERSIONINFOEX structure to determine the Windows edition:
// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
fn edition(version_info: &OSVERSIONINFOEX) -> Option<String> {
    match (
        version_info.dwMajorVersion,
        version_info.dwMinorVersion,
        version_info.wProductType,
    ) {
        // Windows 10.
        (10, 0, VER_NT_WORKSTATION) => Some("Windows 10"),
        (10, 0, _) => Some("Windows Server 2016"),
        // Windows Vista, 7, 8 and 8.1.
        (6, 3, VER_NT_WORKSTATION) => Some("Windows 8.1"),
        (6, 3, _) => Some("Windows Server 2012 R2"),
        (6, 2, VER_NT_WORKSTATION) => Some("Windows 8"),
        (6, 2, _) => Some("Windows Server 2012"),
        (6, 1, VER_NT_WORKSTATION) => Some("Windows 7"),
        (6, 1, _) => Some("Windows Server 2008 R2"),
        (6, 0, VER_NT_WORKSTATION) => Some("Windows Vista"),
        (6, 0, _) => Some("Windows Server 2008"),
        // Windows 2000, Home Server, 2003 Server, 2003 R2 Server, XP and XP Professional x64.
        (5, 1, _) => Some("Windows XP"),
        (5, 0, _) => Some("Windows 2000"),
        (5, 2, _) if unsafe { GetSystemMetrics(SM_SERVERR2) } == 0 => {
            let mut info: SYSTEM_INFO = unsafe { mem::zeroed() };
            unsafe { GetSystemInfo(&mut info) };

            if Into::<DWORD>::into(version_info.wSuiteMask) & VER_SUITE_WH_SERVER
                == VER_SUITE_WH_SERVER
            {
                Some("Windows Home Server")
            } else if version_info.wProductType == VER_NT_WORKSTATION
                && unsafe { info.u.s().wProcessorArchitecture } == PROCESSOR_ARCHITECTURE_AMD64
            {
                Some("Windows XP Professional x64 Edition")
            } else {
                Some("Windows Server 2003")
            }
        }
        _ => None,
    }
    .map(str::to_string)
}

fn get_proc_address(module: &[u8], proc: &[u8]) -> Option<FARPROC> {
    assert!(
        *module.last().expect("Empty module name") == 0,
        "Module name should be zero-terminated"
    );
    assert!(
        *proc.last().expect("Empty procedure name") == 0,
        "Procedure name should be zero-terminated"
    );

    let handle = unsafe { GetModuleHandleA(module.as_ptr() as LPCSTR) };
    if handle.is_null() {
        log::error!(
            "GetModuleHandleA({}) failed",
            String::from_utf8_lossy(module)
        );
        return None;
    }

    unsafe { Some(GetProcAddress(handle, proc.as_ptr() as LPCSTR)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn version() {
        let info = get();
        assert_eq!(Type::Windows, info.os_type());
    }

    #[test]
    fn get_version_info() {
        let version = version_info();
        assert!(version.is_some());
    }

    #[test]
    fn get_edition() {
        let test_data = [
            (10, 0, VER_NT_WORKSTATION, "Windows 10"),
            (10, 0, 0, "Windows Server 2016"),
            (6, 3, VER_NT_WORKSTATION, "Windows 8.1"),
            (6, 3, 0, "Windows Server 2012 R2"),
            (6, 2, VER_NT_WORKSTATION, "Windows 8"),
            (6, 2, 0, "Windows Server 2012"),
            (6, 1, VER_NT_WORKSTATION, "Windows 7"),
            (6, 1, 0, "Windows Server 2008 R2"),
            (6, 0, VER_NT_WORKSTATION, "Windows Vista"),
            (6, 0, 0, "Windows Server 2008"),
            (5, 1, 0, "Windows XP"),
            (5, 1, 1, "Windows XP"),
            (5, 1, 100, "Windows XP"),
            (5, 0, 0, "Windows 2000"),
            (5, 0, 1, "Windows 2000"),
            (5, 0, 100, "Windows 2000"),
        ];

        let mut info = version_info().unwrap();

        for &(major, minor, product_type, expected_edition) in &test_data {
            info.dwMajorVersion = major;
            info.dwMinorVersion = minor;
            info.wProductType = product_type;

            let edition = edition(&info).unwrap();
            assert_eq!(edition, expected_edition);
        }
    }

    #[test]
    fn get_bitness() {
        let b = bitness();
        assert_ne!(b, Bitness::Unknown);
    }

    #[test]
    #[should_panic(expected = "Empty module name")]
    fn empty_module_name() {
        get_proc_address(b"", b"RtlGetVersion\0");
    }

    #[test]
    #[should_panic(expected = "Module name should be zero-terminated")]
    fn non_zero_terminated_module_name() {
        get_proc_address(b"ntdll", b"RtlGetVersion\0");
    }

    #[test]
    #[should_panic(expected = "Empty procedure name")]
    fn empty_proc_name() {
        get_proc_address(b"ntdll\0", b"");
    }

    #[test]
    #[should_panic(expected = "Procedure name should be zero-terminated")]
    fn non_zero_terminated_proc_name() {
        get_proc_address(b"ntdll\0", b"RtlGetVersion");
    }

    #[test]
    fn proc_address() {
        let address = get_proc_address(b"ntdll\0", b"RtlGetVersion\0");
        assert!(address.is_some());
    }

    #[test]
    fn get_product_name() {
        let edition = product_name().expect("edition() failed");
        assert!(!edition.is_empty());
    }

    #[test]
    fn to_wide_str() {
        let data = [
            ("", [0x0000].as_ref()),
            ("U", &[0x0055, 0x0000]),
            ("你好", &[0x4F60, 0x597D, 0x0000]),
        ];

        for (s, expected) in &data {
            let wide = to_wide(s);
            assert_eq!(&wide, expected);
        }
    }
}
