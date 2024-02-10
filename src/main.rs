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
 
 fn total_ips_from_subnet(prefix_length: u32) -> u32 {
     2_u32.pow(32-prefix_length)
 }
 
 fn total_ips_from_cidr(cidr_block : &str) -> u32{
     let subnet_parts : Vec<&str> = cidr_block.split('/').collect();
     let host_bits : u32 = subnet_parts[1].parse::<u32>().expect("");
 
     2_u32.pow(32-host_bits)
 
 }
 
 
 
 fn get_subnet_mask_bits(cidr_block : &str) -> String{
     let mut subnet_mask = String::new();
     let subnet_parts: Vec<&str> = cidr_block.split('/').collect();
     let host_bits : i32 = subnet_parts[1].parse::<i32>().expect("Unable to parse CIDR block");
     for n in 1..33{
         if n <= host_bits{
             subnet_mask.push('1');
         } else {
             subnet_mask.push('0');
         }
         if (n % 8) == 0 && n != 32{
             subnet_mask.push('.');
         }
     }
     return subnet_mask;
 }
 
 
 fn get_subnet_mask_dec(cidr_block: &str) -> Vec<u8> {
     let subnet_parts: Vec<&str> = cidr_block.split('/').collect();
     let prefix_length: u8 = subnet_parts[1].parse().expect("Unable to parse prefix length");
 
     // Initialize the subnet mask as a 32-bit integer.
     let mask: u32 = 0xFFFFFFFF << (32 - prefix_length);
 
     // Convert the mask into a Vec<u8> representation.
     let mut subnet_mask: Vec<u8> = Vec::new();
     for i in (0..4).rev() {
         subnet_mask.push(((mask >> (i * 8)) & 0xFF) as u8);
     }
     println!("Subnet Mask: {}.{}.{}.{}", subnet_mask[0], subnet_mask[1], subnet_mask[2], subnet_mask[3]);
     subnet_mask
 }
 
 
 fn get_network_start(cidr_block : Vec<u8>, subnet_mask : Vec<u8>) -> Vec<u8> {
 
     let mut network_start: Vec<u8> = Vec::new();
     for i in 0..4{
         network_start.push(cidr_block[i] & subnet_mask[i]);
     }
 
     println!("Network start: {}.{}.{}.{}", network_start[0], network_start[1], network_start[2], network_start[3]);
     network_start
 }
 
 fn cidr_str_to_network_vec(cidr_block: &str) -> Vec<u8> {
     // Split the CIDR block to get the IP address part.
     let ip_address_part = cidr_block.split('/').next().expect("Invalid CIDR block format");
 
     // Split the IP address into its octets and parse each one into a u8.
     let cidr_block_vec: Vec<u8> = ip_address_part
         .split('.')
         .map(|octet| octet.parse::<u8>().expect("Invalid IP address format"))
         .collect();
 
     cidr_block_vec
 }
 
 
 
 fn list_ips_from_subnet(cidr_block : &str){
     println!("{}", cidr_block);
 
     let subnet_mask : Vec<u8> = get_subnet_mask_dec(cidr_block);
 
     let network_vec = cidr_str_to_network_vec(cidr_block);
 
     let network_start: Vec<u8> = get_network_start(network_vec, subnet_mask);
     let total_ips = total_ips_from_cidr(cidr_block);
 
     let mut network_address = u32::from(network_start[0]) << 24
         | u32::from(network_start[1]) << 16
         | u32::from(network_start[2]) << 8
         | u32::from(network_start[3]);
 
     for _ in 0..total_ips {
         // Convert back to Vec<u8> to print
         let ip_address = vec![
             ((network_address >> 24) & 0xFF) as u8,
             ((network_address >> 16) & 0xFF) as u8,
             ((network_address >> 8) & 0xFF) as u8,
             (network_address & 0xFF) as u8,
         ];
 
         println!("{}.{}.{}.{}", ip_address[0], ip_address[1], ip_address[2], ip_address[3]);
 
         network_address += 1;
     }
 }
 
 
 fn main() {
    let args: Vec<String> = env::args().collect();
 
    if args.len() < 2 || (&args[1] == "-h"){
         println!("{}", USAGE);
         return;
    }
 
 
     let subnet = &args[1];
     println!("Input subnet in CIDR notation: {}", subnet);
     let subnet_parts: Vec<&str> = subnet.split('/').collect(); // Collect into a Vec for indexing
 
     if subnet_parts.len() == 2 {
         // Print the parts
         println!("Network address: {}", subnet_parts[0]);
         println!("Prefix length: {}", subnet_parts[1]);
 
         // Parse prefix length
         let host_bits_prefix: u32 = subnet_parts[1].parse::<u32>().expect("Unable to parse CIDR block");
 
         println!("Host bits prefix: {}", host_bits_prefix);
         println!("Total number of available hosts: {}", total_ips_from_subnet(host_bits_prefix));
     } else {
         println!("Invalid CIDR notation.");
     }
     let subnet_mask : String = get_subnet_mask_bits(subnet);
     println!("Subnet mask of {} : {}", subnet, subnet_mask);
     println!("Available IPs within the range: {}", subnet);
     list_ips_from_subnet(subnet);
 }
 
 
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_total_ips_from_subnet() {
         assert_eq!(total_ips_from_subnet(24), 256);
         assert_eq!(total_ips_from_subnet(30), 4);
     }
 
     #[test]
     fn test_get_subnet_mask_dec() {
         // Testing with a /24 prefix
         let mask_24 = get_subnet_mask_dec("192.168.1.0/24");
         assert_eq!(mask_24, vec![255, 255, 255, 0]);
 
         // Testing with a /16 prefix
         let mask_16 = get_subnet_mask_dec("192.168.0.0/16");
         assert_eq!(mask_16, vec![255, 255, 0, 0]);
     }
 
     #[test]
     fn test_cidr_str_to_network_vec() {
         let cidr_block = "192.168.1.1/24";
         let expected_vec = vec![192, 168, 1, 1];
         assert_eq!(cidr_str_to_network_vec(cidr_block), expected_vec);
 
         let cidr_block = "10.0.0.1/16";
         let expected_vec = vec![10, 0, 0, 1];
         assert_eq!(cidr_str_to_network_vec(cidr_block), expected_vec);
     }
 
     #[test]
     fn test_get_network_start() {
         let ip_vec = vec![192, 168, 1, 10];
         let subnet_mask = vec![255, 255, 255, 0];
         let expected_network_start = vec![192, 168, 1, 0];
         assert_eq!(get_network_start(ip_vec, subnet_mask), expected_network_start);
 
         let ip_vec = vec![10, 0, 0, 15];
         let subnet_mask = vec![255, 255, 0, 0];
         let expected_network_start = vec![10, 0, 0, 0];
         assert_eq!(get_network_start(ip_vec, subnet_mask), expected_network_start);
     }
 
 
 }