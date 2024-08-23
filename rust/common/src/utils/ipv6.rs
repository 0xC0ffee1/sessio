
use std::net::Ipv6Addr;

use if_addrs::get_if_addrs;


//@TODO Maybe make flutter send device ipv6
pub fn get_first_global_ipv6() -> Option<Ipv6Addr> {
    let ifaces = get_if_addrs().expect("Failed to get network interfaces");

    ifaces.into_iter()
        .filter_map(|iface| match iface.addr.ip() {
            std::net::IpAddr::V6(ipv6) => {
                if !iface.addr.is_link_local() && !iface.addr.is_loopback() {
                    Some(ipv6)
                } else {
                    None
                }
            }
            _ => None,
        })
        .next()
}
