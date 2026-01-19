#[derive(Debug)]  // Add this to print the enum
enum IpAddrKind {
    V4,
    V6,
}

// Add this to print the struct
#[derive(Debug)]
// Different attribute types must be separated, I could include #[derive(Debug, Clone, Copy, PartialEq)] in the above, but not allow(x, y, z)
#[allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Use {:?} for Debug printing, and fix the missing }
    println!("I'm home {:?}", home);
    println!("I'm loopback {:?}", loopback);
}