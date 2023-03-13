#![allow(dead_code, unused_imports)]
use std::env;
use std::process::exit;

use dump::packet;
mod dump;


fn main (){
    let args: Vec<String> = env::args().collect();
    if args.len()<3{
        println!("usage: {} DevName",args[1]);
        println!("usage: {} Plug ",args[2]);
        println!("usage: {} Port",args[3]);
        exit(1);
    }
    println!("device:{} plug:{} port:{} ",args[1].as_str(),args[2].as_str(),args[3].as_str());
    let mut pkg = libpcap::open(args[1].as_str());

    let port_fmt = format!("tcp and port {}", args[3].as_str());

    libpcap::setfilter(&mut pkg,&port_fmt);
    parser(&mut pkg,args[2].to_string());

    libpcap::close(&mut pkg);
}


pub fn to_bytes(input: &[i32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4 * input.len());

    for value in input {
        bytes.extend(&value.to_be_bytes());
    }

    bytes
}



fn parser(pkg: &mut libpcap::Packet,plug:String) {
    loop {
        match libpcap::next_ex(pkg) {
            _ => {
                let mut raw:Vec<u8> = vec![];
                unsafe {
                    for i in 0..pkg.head.len{
                        let a = pkg.data.offset(i as isize); 
                        raw.push(*a);
                    }
                }

                match  dump::dumper::Dumper::from_bytes(&raw,plug.to_string()) {
                    Ok(p) => {
                        let p1 = p.clone();
                        packet::Packet::debug(p,p1.get_s2_d()); 
                    }
                    Err(_) => (),
                }                                        
            }                  
        }
    }
}
