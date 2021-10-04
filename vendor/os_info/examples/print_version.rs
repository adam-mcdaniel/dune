fn main() {
    let info = os_info::get();

    // Print full information:
    println!("OS information: {}", info);

    // Print information separately:
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());
    println!("Edition: {:?}", info.edition());
    println!("Codename: {:?}", info.codename());
    println!("Bitness: {}", info.bitness());
}
