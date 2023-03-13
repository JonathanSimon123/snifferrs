
use std::fmt::Debug;
use std::net::TcpStream;
use httparse::Request;
use httparse::Header as OtherHeader;
use super::error::DumpErr;
use std::{fmt, str};
use std::path::Path;
extern crate colored; // not needed in Rust 2018+
use colored::*;
use std::error::Error;



pub struct HttpParser {}

pub struct  HttpHdr {
    pub method: Option<String>,
    pub uri:Option<String>,
    pub header:String,  
}

impl Clone for HttpHdr {
    fn clone(&self) -> Self {
        Self { method: self.method.clone(), uri: self.uri.clone(), header: self.header.clone() }
    }
}


impl Debug for HttpHdr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _result =write!(f," {}:{} \n {}:{}\n {}:{} \n ","Method".italic().yellow(),self.method.as_ref().unwrap(),"URI".italic().yellow(),self.uri.as_ref().unwrap(),"Header".italic().yellow(),self.header);
        return Ok(());
    }
}

impl HttpHdr {
    pub fn new(method: Option<String>, uri: Option<String>, header:String) -> Self { Self { method, uri, header } }
}

impl HttpParser {
    pub fn parse_http_hdr(buffer: &[u8])-> Result<HttpHdr,DumpErr> {
       let mut http_hdr =  HttpHdr::new(Some("".to_string()),Some("".to_string()),"".to_string());

        if buffer.len() == 0 {
            return Err(DumpErr { info: String::from("") });
        }
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        if let Ok(_) = req.parse(buffer) {
            for hdr in req.headers.iter(){
                let hdr_str = format!("{}:{} ",hdr.name,String::from_utf8_lossy(hdr.value));
                http_hdr.header += &hdr_str;
            }
        
            http_hdr.method = Some(req.method.unwrap().to_string());
            http_hdr.uri = Some(req.path.unwrap().to_string());
     
            return Ok(http_hdr);
        }
        
  
        return Err(DumpErr { info: String::new() });
    }
}
