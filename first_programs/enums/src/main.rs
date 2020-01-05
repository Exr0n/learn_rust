enum IpAddrKind {
    V6 {
        addr: String
    },
    V4(u8, u8, u8, u8),
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Hello, world!");
}
