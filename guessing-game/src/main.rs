use rand::Rng;
use std::cmp::Ordering;
use std::io;

enum IpAddVersion {
    IPv4(String),
    IPv6(String),
}
struct IpValidation {
    address: String,
    v: IpAddVersion,
}
fn routes(ip: &str, v: IpAddVersion::IPv4) {
    let mut routes = Vec::new();
    let mut ip_parts: Vec<&str> = ip.split('.').collect();
    for part in ip_parts {
        if let Ok(num) = part.parse::<u8>() {
            routes.push(num);
        } else {
            return;
        }
    }
    if routes.len() == 4 {
        println!("Valid IPv4 address");
    } else {
        println!("Invalid IPv4 address");
    }
}
fn main() {
    // let ket = IpAddVersion::IPv4;
    // let ip = "237.84.2.178";


    // routes(ip, ket);


    let mut vec: Vec<u32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
}
// fn main() {

//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("{secret_number}");
//     println!("Enter a random number between 1 and 100");

//     loop {
//         let mut input: String = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
//         let input: u32 = match input.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Not a number");
//                 continue;
//             }
//         };

//         match input.cmp(&secret_number) {
//             Ordering::Less => println!("Too small"),
//             Ordering::Equal => {
//                 println!("Won");
//                 break;
//             }
//             Ordering::Greater => println!("Too big"),
//         }
//         println!("You typed: {}", input);
//     }
// }
