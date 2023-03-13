
use crate::dump::packet::Packet;

use super::error::DumpErr;


pub struct Dumper {
}

impl Dumper {
    pub fn from_bytes(b: &[u8],plug:String) ->Result<Packet,DumpErr>  {

        match  Packet::from_bytes(b,plug){
           Ok(pkg) => {
               return Ok(pkg); 
           }
            Err(e) => {
                println!("{}",e);
                return  Err(e);
            }
           
        }       
    }
}

