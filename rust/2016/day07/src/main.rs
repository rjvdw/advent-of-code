use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::ipv7::IPv7;

mod ipv7;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let ips = File::open(&args[1]).read_lines(1).collect::<Vec<IPv7>>();

    let ips_that_support_tls = ips.iter().filter(|ip| ip.supports_tls()).count();
    println!("There are {} IPs that support TLS.", ips_that_support_tls);

    let ips_that_support_ssl = ips.iter().filter(|ip| ip.supports_ssl()).count();
    println!("There are {} IPs that support SSL.", ips_that_support_ssl);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_tls() {
        assert!(ipv7("abba[mnop]qrst").supports_tls());
        assert!(!ipv7("abcd[bddb]xyyx").supports_tls());
        assert!(!ipv7("aaaa[qwer]tyui").supports_tls());
        assert!(ipv7("ioxxoj[asdfgh]zxcvbn").supports_tls());
    }

    #[test]
    fn test_supports_ssl() {
        assert!(ipv7("aba[bab]xyz").supports_ssl());
        assert!(!ipv7("xyx[xyx]xyx").supports_ssl());
        assert!(ipv7("aaa[kek]eke").supports_ssl());
        assert!(ipv7("zazbz[bzb]cdb").supports_ssl());
    }

    fn ipv7(s: &str) -> IPv7 {
        s.parse().unwrap()
    }
}
