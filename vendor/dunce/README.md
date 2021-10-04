# Dunce (de-UNC)

In Windows the regular paths (`C:\foo`) are supported by all programs,
but have lots of bizarre restrictions for backwards compatibility with MS-DOS.
There are also Windows NT UNC paths (`\\?\C:\foo`), which are more robust and with fewer gotchas,
but are rarely supported by Windows programs. Even Microsoft's own!

This crate converts Windows UNC paths to the MS-DOS-compatible format whenever possible,
but leaves UNC paths as-is when they can't be unambiguously expressed in a simpler way.
This allows legacy programs to access all paths they can possibly access,
and doesn't break any paths for UNC-aware programs.

In Rust the worst UNC offender is the `fs::canonicalize()` function. This crate provides
a drop-in replacement for it that returns paths you'd expect.

On non-Windows platforms these functions leave paths unmodified, so it's safe to use them
unconditionally for all platforms.
