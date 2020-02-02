enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum EnumIpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: four,
        address: "127.0.0.1".to_string(),
    };

    let loopback = IpAddr { kind: IpAddrKind::V6, address: "::1".to_string() };

    let home = EnumIpAddr("127.0.0.1".to_string());
    let loopback = EnumIpAddr("::1".to_string());
}

