/*
 * Basic subnet expansion tool in Rust to learn Rust!
 * By: sc1341
 * */

use std::env;

const USAGE: &str = "
Usage: subnet_helper [OPTIONS] CIDR_BLOCK
Options:
  -h, --help      Show this help message and exit.
  -v, --version   Display the version information.

Examples:
  subnet_helper 192.168.1.0/24   List all IP addresses in the 192.168.1.0 subnet.
  subnet_helper -h               Show the help message.
";

pub struct Subnet {
    cidr_block: String,
    subnet_mask: Vec<u8>,
    network_start: Vec<u8>,
    total_ips: u32,
}

impl Subnet {
    pub fn new(cidr_block: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = cidr_block.split('/').collect();
        if parts.len() != 2 {
            return Err("CIDR block must be in the format IP/PREFIX".into());
        }
        let ip_address = Self::cidr_str_to_network_vec(cidr_block);
        let subnet_mask = Self::get_subnet_mask_dec(cidr_block)?;
        let network_start = Self::get_network_start(&ip_address, &subnet_mask)?;
        let total_ips = Self::total_ips_from_cidr(cidr_block)?;

        Ok(Subnet {
            cidr_block: cidr_block.to_string(),
            subnet_mask,
            network_start,
            total_ips,
        })
    }

    fn total_ips_from_cidr(cidr_block: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = cidr_block.split('/').collect();
        let prefix_length: u32 = parts[1].parse()?;
        Ok(2_u32.pow(32 - prefix_length))
    }

    fn get_subnet_mask_dec(cidr_block: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = cidr_block.split('/').collect();
        let prefix_length: u32 = parts[1].parse()?;
        let mask: u32 = (!0u32).checked_shl(32 - prefix_length).unwrap_or(0);
        Ok(mask.to_be_bytes().to_vec())
    }

    fn get_network_start(ip_address: &Vec<u8>, subnet_mask: &Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        ip_address.iter().zip(subnet_mask.iter())
            .map(|(&ip, &mask)| Ok(ip & mask))
            .collect()
    }

    fn cidr_str_to_network_vec(cidr_block: &str) -> Vec<u8> {
        cidr_block.split('/').next().unwrap()
            .split('.').map(|octet| octet.parse().unwrap()).collect()
    }

    pub fn list_ips(&self) {
        let mut current_ip = u32::from_be_bytes([self.network_start[0], self.network_start[1], self.network_start[2], self.network_start[3]]);
        for _ in 0..self.total_ips {
            let ip_bytes = current_ip.to_be_bytes();
            println!("{}.{}.{}.{}", ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
            current_ip = current_ip.checked_add(1).expect("IP address overflow");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1] == "-h" || args[1] == "--help" {
        println!("{}", USAGE);
        return;
    }

    let cidr_block = &args[1];
    match Subnet::new(cidr_block) {
        Ok(subnet) => {
            subnet.list_ips();
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_ips_from_cidr() {
        let cidr_block_24 = "192.168.1.0/24";
        assert_eq!(Subnet::total_ips_from_cidr(cidr_block_24).unwrap(), 256);

        let cidr_block_30 = "192.168.1.0/30";
        assert_eq!(Subnet::total_ips_from_cidr(cidr_block_30).unwrap(), 4);
    }

    #[test]
    fn test_get_subnet_mask_dec() {
        let cidr_block_24 = "192.168.1.0/24";
        assert_eq!(Subnet::get_subnet_mask_dec(cidr_block_24).unwrap(), vec![255, 255, 255, 0]);

        let cidr_block_16 = "192.168.0.0/16";
        assert_eq!(Subnet::get_subnet_mask_dec(cidr_block_16).unwrap(), vec![255, 255, 0, 0]);
    }

    #[test]
    fn test_get_network_start() {
        let ip_address = vec![192, 168, 1, 10];
        let subnet_mask = vec![255, 255, 255, 0];
        let expected_network_start = vec![192, 168, 1, 0];
        assert_eq!(Subnet::get_network_start(&ip_address, &subnet_mask).unwrap(), expected_network_start);

        let ip_address = vec![10, 0, 0, 15];
        let subnet_mask = vec![255, 0, 0, 0];
        let expected_network_start = vec![10, 0, 0, 0];
        assert_eq!(Subnet::get_network_start(&ip_address, &subnet_mask).unwrap(), expected_network_start);
    }

    #[test]
    fn test_cidr_str_to_network_vec() {
        let cidr_block = "192.168.1.1/24";
        let expected_vec = vec![192, 168, 1, 1];
        assert_eq!(Subnet::cidr_str_to_network_vec(cidr_block), expected_vec);

        let cidr_block = "10.0.0.1/16";
        let expected_vec = vec![10, 0, 0, 1];
        assert_eq!(Subnet::cidr_str_to_network_vec(cidr_block), expected_vec);
        
        let cidr_block = "23.165.24.1/24";
        let expected_vec = vec![23, 165, 24, 1];
        assert_eq!(Subnet::cidr_str_to_network_vec(cidr_block), expected_vec);
    }
}
