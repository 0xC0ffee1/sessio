use std::net::{SocketAddr, SocketAddrV6, Ipv6Addr, Ipv4Addr};

pub mod streams;
pub mod keygen;
pub mod ipv6;
pub mod events;

pub fn map_ipv4_to_ipv6(socket_addr: SocketAddr) -> SocketAddr {
    match socket_addr {
        SocketAddr::V4(v4_addr) => {
            // Extract the IPv4 address
            let ipv4 = v4_addr.ip();

            // Map the IPv4 address to an IPv6-mapped IPv4 address
            let ipv6_mapped = ipv4_to_ipv6(*ipv4);

            // Create a new SocketAddrV6 with the mapped IPv6 address and the same port
            SocketAddr::V6(SocketAddrV6::new(
                ipv6_mapped,
                v4_addr.port(),
                0,
                0,
            ))
        }
        SocketAddr::V6(_) => {
            // If it's already an IPv6 address, return it as-is
            socket_addr
        }
    }
}

fn ipv4_to_ipv6(ipv4: Ipv4Addr) -> Ipv6Addr {
    Ipv6Addr::from(u128::from_be_bytes([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff,
        ipv4.octets()[0], ipv4.octets()[1], 
        ipv4.octets()[2], ipv4.octets()[3],
    ]))
}