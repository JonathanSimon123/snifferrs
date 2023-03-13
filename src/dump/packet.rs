

use std::collections::HashMap;
use std::fmt::Error;
use std::clone::Clone;
use lazy_static::lazy_static;
extern crate colored; // not needed in Rust 2018+
use colored::*;
use crate::dump::packet::Layer4::{ICMP, UDP,TCP};
use crate::dump::packet::Plug::{MYSQL,HTTP};
use crate::dump::packet_parser::{EtherParser, IcmpParser, Ip4Parser, TcpParser, UdpParser};
use crate::dump::mysql::{MysqlHdr,MysqlParser,};
use crate::dump::http::{HttpParser,HttpHdr};


use super::error::{DumpErr, self};

const ETH_TYPE_IP: u16 = 0x0800;
const ETH_TYPE_ARP: u16 = 0x0806;
const IP_PROTO_ICMP: u8 = 0x01;
const IP_PROTO_TCP: u8 = 0x06;
const IP_PROTO_UDP: u8 = 0x11;
const ICMP_TYPE_ECHO_REPLY: u8 = 0x0;
const ICMP_TYPE_ECHO_REQUEST: u8 = 0x8;
const INVALID_PLAYLOAD: &str = r#" header len too short"#;

lazy_static! {
    static ref ETH_TYPE_MAP: HashMap<u16, &'static str> = {
        let map = HashMap::from([
            (ETH_TYPE_IP ,"ipv4"),
            (ETH_TYPE_ARP ,"arp"),
        ]);
        return map;
    };

    static ref IP_PROTO_MAP: HashMap<u8, &'static str> = {
        let map = HashMap::from([
            (IP_PROTO_ICMP ,"icmp"),
            (IP_PROTO_TCP ,"tcp"),
            (IP_PROTO_UDP ,"udp"),
        ]);
        return map;
    };
 static ref ICMP_TYPE_MAP: HashMap<u8, &'static str> = {
        let map = HashMap::from([
            (ICMP_TYPE_ECHO_REPLY ,"icmp ping reply"),
            (ICMP_TYPE_ECHO_REQUEST ,"icmp ping request"),
        ]);
        return map;
    };
}

enum Layer4 {
    //icmp 原则上不算4层,这里只是方便分类
    ICMP(IcmpData),
    UDP(UdpHdr),
    TCP(TcpHdr),
}

impl Copy for Layer4 {
    
}


impl Clone for Layer4 {
    fn clone(&self) -> Self {
        match self {
            Self::ICMP(arg0) => Self::ICMP(arg0.clone()),
            Self::UDP(arg0) => Self::UDP(arg0.clone()),
            Self::TCP(arg0) => Self::TCP(arg0.clone()),
        }
    }
}




enum Plug {
    MYSQL(MysqlHdr),
    HTTP(HttpHdr),
}



impl Clone for Plug {
    fn clone(&self) -> Self {
        match self {
            Self::MYSQL(arg0) => Self::MYSQL(arg0.clone()),
            Self::HTTP(arg0) => Self::HTTP(arg0.clone()),
        }
    }
}




impl std::fmt::Debug for Plug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MYSQL(mysql_hdr) => f.debug_tuple("MYSQL").field(mysql_hdr).finish(),
            Self::HTTP(http_hdr) => f.debug_tuple("HTTP").field(http_hdr).finish(),
        }
    }
}



// 完整数据包, 由原始字节数组逐层解析, 遇到错误则停止解析,并将错误写到ext_info
pub struct Packet {
    eth_hdr: Option<EthHdr>,
    ip_hdr: Option<IP4Hdr>,
    l4: Option<Layer4>,
    plug: Option<Plug>,
    ext_info: Option<String>,
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        Self { eth_hdr: self.eth_hdr.clone(), ip_hdr: self.ip_hdr.clone(), l4: self.l4.clone(), plug: self.plug.clone(), ext_info: self.ext_info.clone() }
    }
}


impl Packet {
    // 原始字节数组解析包
    pub fn from_bytes(b: &[u8],plug:String) ->Result<Packet,DumpErr>  {
      
        let mut packet = Packet {
            eth_hdr: None,
            ip_hdr: None,
            l4: None,
            plug:None,
            ext_info: None,
        };
        let eth_type;
        let  ip_proto;
        //解析以太网头部
        match EtherParser::parse_eth_hdr(b) {            
            Ok(e) => {
                eth_type = e.proto;
                packet.eth_hdr = Some(e);
            }
            Err(e) => {
                return Err(e);
            }
        }

        let mut off: usize = 14; // 字节偏移
        match eth_type {
            ETH_TYPE_IP => {
                //解析ip头部
                match Ip4Parser::parse_ip4_hdr(&b[off..]) {
                    Ok(ip) => {

                        ip_proto = ip.proto;
                        off += (ip.ihl as usize) * 4;
                        
                        packet.ip_hdr = Some(ip);
                        if off > b.len() {
                            return Err(DumpErr { info: "off>len".to_string() });
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            _ => {

                // TODO  非eth 
                println!("not ip frame");
             
                //不是ip头 停止解析
                return Err(DumpErr { info: String::from("not ip frame") });
            }
        }
        match ip_proto {
            IP_PROTO_ICMP => {

                //解析icmp包
                match IcmpParser::parse_icmp(&b[off..]) {
                    Ok(icmp) => {

                        packet.l4 = Some(ICMP(icmp));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            IP_PROTO_UDP => {

                //解析udp头
                match UdpParser::parse_udp(&b[off..]) {
                    Ok(udp) => {
                        packet.l4 = Some(UDP(udp));
                        off += 8;
                        if off > b.len() {
                            return Err(DumpErr{info:INVALID_PLAYLOAD.to_string()});
                        }

                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            IP_PROTO_TCP => {

                //解析tcp头
                match TcpParser::parse_tcp(&b[off..]) {
                    Ok(tcp) => {
                        // let data =  &b[off+20..(tcp.data_off as usize) * 4]                       
                    
                        off += 20;  
              
                        if b.len() < off {
                            return  Err(DumpErr{info:INVALID_PLAYLOAD.to_string()});
                        }else {
                        }
                               
                        packet.l4 = Some(TCP(tcp));

                        if plug != "mysql" {
                            match HttpParser::parse_http_hdr(&b[off..]) {
                                Ok(http) => {
                                    packet.plug = Some(HTTP(http));
                                    return  Ok(packet);
                                },
                                Err(_)=> {}
                            }  
                        }else {
                            match MysqlParser::parse_mysql(&b[off..]) {
                                Ok(mysql) => {
                                    packet.plug = Some(MYSQL(mysql));
                                    return  Ok(packet);
                                }
                                Err(_)=> {}
                            }  
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            _ => {}
        }

        return Ok(packet);
    }  
    
    pub fn get_s2_d(self) -> String{
       if let Some(TCP(tcphdr)) = self.l4 {
         return  format!("【{}:{}==>{}:{}】 ",self.ip_hdr.unwrap().get_saddr(),tcphdr.sport,self.ip_hdr.unwrap().get_daddr(),tcphdr.dport);           
       } else {
                return "".to_string();
               } 
    }

    pub fn debug(self,s2d: String) {

        match self.plug {
            Some(MYSQL(mysql)) => {
                println!("{} {}","MYSQL".purple().magenta(),s2d);
                println!("{:?}",mysql);
            }
            Some(HTTP(http)) => {
                println!("{} {}","HTTP".purple().magenta(),s2d);
                println!("{:?}",http);
            }
            _=>{

            }                
        } 
    }
}

#[derive(Clone)]
pub struct EthHdr {
    pub d_mac: [u8; 6],
    pub s_mac: [u8; 6],
    pub proto: u16,
}

impl Copy for EthHdr {}


#[derive(Debug, Clone)]
pub struct IP4Hdr {
    pub version: u8,
    pub ihl: u8,
    pub tos: u8,
    pub enc_f0: bool,
    pub enc_f1: bool,
    pub total_len: u16,
    pub id: u16,
    pub flag_re: bool,
    pub flag_df: bool,
    pub flag_mf: bool,
    pub frag_off: u16,
    pub ttl: u8,
    pub proto: u8,
    pub checksum: u16,
    pub saddr: u32,
    pub daddr: u32,
}

impl Copy for IP4Hdr {}

impl IP4Hdr {
    pub fn new_with_zero() -> IP4Hdr {
        return IP4Hdr {
            version: 0,
            ihl: 0,
            tos: 0,
            enc_f0: false,
            enc_f1: false,
            total_len: 0,
            id: 0,
            flag_re: false,
            flag_df: false,
            flag_mf: false,
            frag_off: 0,
            ttl: 0,
            proto: 0,
            checksum: 0,
            saddr: 0,
            daddr: 0,
        };
    }
    pub fn get_saddr(&self) -> String {
       let mut s = String::new();
       s.push_str(format!("{}.{}.{}.{}",(self.saddr>>24) as u8,(self.saddr>>16) as u8,(self.saddr>>8) as u8,(self.saddr) as u8).as_str());
       return  s;
    }

    pub fn get_daddr(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("{}.{}.{}.{}",(self.daddr>>24) as u8,(self.daddr>>16) as u8,(self.daddr>>8) as u8 ,self.daddr as u8).as_str());
        return  s;
    }
}


#[derive(Clone)]
pub struct IcmpData(pub u8, pub u8, pub u16);

impl Copy for IcmpData {
    
}

impl IcmpData {}

#[derive(Clone)]
pub struct UdpHdr {
    pub sport: u16,
    pub dport: u16,
    pub len: u16,
    pub checksum: u16,
}

impl Copy for UdpHdr {
    
}


pub struct TcpHdr {
    pub sport: u16,
    pub dport: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub data_off: u8,
    pub flag_r0: bool,
    pub flag_r1: bool,
    pub flag_r2: bool,
    pub flag_ns: bool,
    pub flag_cwr: bool,
    pub flag_ece: bool,
    pub flag_ugr: bool,
    pub flag_ack: bool,
    pub flag_psh: bool,
    pub flag_rst: bool,
    pub flag_syn: bool,
    pub flag_fin: bool,
    pub win_size: u16,
    pub checksum: u16,
    pub urg_p: u16,
}

impl Copy for TcpHdr {
    
}

impl Clone for TcpHdr {
    fn clone(&self) -> Self {
        Self { sport: self.sport.clone(), dport: self.dport.clone(), seq_num: self.seq_num.clone(), ack_num: self.ack_num.clone(), data_off: self.data_off.clone(), flag_r0: self.flag_r0.clone(), flag_r1: self.flag_r1.clone(), flag_r2: self.flag_r2.clone(), flag_ns: self.flag_ns.clone(), flag_cwr: self.flag_cwr.clone(), flag_ece: self.flag_ece.clone(), flag_ugr: self.flag_ugr.clone(), flag_ack: self.flag_ack.clone(), flag_psh: self.flag_psh.clone(), flag_rst: self.flag_rst.clone(), flag_syn: self.flag_syn.clone(), flag_fin: self.flag_fin.clone(), win_size: self.win_size.clone(), checksum: self.checksum.clone(), urg_p: self.urg_p.clone() }
    }
}

impl TcpHdr {
    pub fn new_from_zero() -> TcpHdr {
        return TcpHdr {
            sport: 0,
            dport: 0,
            seq_num: 0,
            ack_num: 0,
            data_off: 0,
            flag_r0: false,
            flag_r1: false,
            flag_r2: false,
            flag_ns: false,
            flag_cwr: false,
            flag_ece: false,
            flag_ugr: false,
            flag_ack: false,
            flag_psh: false,
            flag_rst: false,
            flag_syn: false,
            flag_fin: false,
            win_size: 0,
            checksum: 0,
            urg_p: 0,
        };
    }
}


