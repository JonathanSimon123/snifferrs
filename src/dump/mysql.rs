#![allow(dead_code, unused_imports)]
use crate::dump::error::DumpErr;
extern crate colored; // not needed in Rust 2018
use colored::*;

pub struct MysqlParser {}

pub struct MysqlHdr {
    pub cmd: Option<Cmd>,
    pub data: Vec<u8>,
}



impl Clone for MysqlHdr {
    fn clone(&self) -> Self {
        Self { cmd: self.cmd.clone(), data: self.data.clone() }
    }
}

impl std::fmt::Debug for MysqlHdr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _result =write!(f," {}:{:?} \n {}:{}\n ","COM".italic().yellow(),self.cmd,"SQL".italic().yellow(),String::from_utf8_lossy(self.data.as_ref()));
        Ok(())
    }
}

impl MysqlHdr {
    pub fn to_string(self) -> Result<String,DumpErr> {
        let  data = self.data.iter().map(|&c| c as char).collect::<String>();  
       
        let mut sql_string: String = String::new();
        match self.cmd.as_ref().unwrap(){
            Cmd::ComInitDB =>{
                sql_string.push_str(&format!("USE {}",data));
            },
            Cmd::ComSleep => todo!(),
            Cmd::ComQuit => todo!(),
            Cmd::ComQuery|Cmd::ComCreateDb => {
                sql_string.push_str(&data)
            },
            Cmd::ComFieldList => todo!(),
            Cmd::ComDropDb =>{
                sql_string.push_str(&format!("Drop DB {}",data));
            },
            Cmd::ComRefresh => todo!(),
            Cmd::ComShutdown => todo!(),
            Cmd::ComStatistics => todo!(),
            Cmd::ComProcessInfo => todo!(),
            Cmd::ComConnect => todo!(),
            Cmd::ComProcessKill => todo!(),
            Cmd::ComDebug => todo!(),
            Cmd::ComPing => todo!(),
            Cmd::ComTime => todo!(),
            Cmd::ComDelayedInsert => todo!(),
            Cmd::ComChangeUser => todo!(),
            Cmd::ComBinlogDump => todo!(),
            Cmd::ComTableDump => todo!(),
            Cmd::ComConnectOut => todo!(),
            Cmd::ComRegisterSlave => todo!(),
            Cmd::ComStmtPrepare => {
                if self.data.len() <= 5 {
                    return Err(DumpErr{info:"".to_owned()});
                }
                sql_string =   self.data[0..5].iter().map(|&c| c as char).collect::<String>(); 
            },
            Cmd::ComStmtExecute => todo!(),
            Cmd::ComStmtSendLongData => todo!(),
            Cmd::ComStmtClose => todo!(),
            Cmd::ComStmtReset => todo!(),
            Cmd::ComSetOption => todo!(),
            Cmd::ComStmtFetch => todo!(),
            Cmd::ComDaemon => todo!(),
            Cmd::ComBinlogDumpGtid => todo!(),
            Cmd::ComResetConnection => todo!(),                 
        }

        return  Ok(sql_string);

    }
}


pub enum Cmd  {
    ComSleep = 0,
	ComQuit,
	ComInitDB,
	ComQuery,
	ComFieldList,
	ComCreateDb,
	ComDropDb,
	ComRefresh,
	ComShutdown,
	ComStatistics,
	ComProcessInfo,
	ComConnect,
	ComProcessKill,
	ComDebug,
	ComPing,
	ComTime,
	ComDelayedInsert,
	ComChangeUser,
	ComBinlogDump,
	ComTableDump,
	ComConnectOut,
	ComRegisterSlave,
	ComStmtPrepare,
	ComStmtExecute,
	ComStmtSendLongData,
	ComStmtClose,
	ComStmtReset,
	ComSetOption,
	ComStmtFetch,
	ComDaemon,
	ComBinlogDumpGtid,
	ComResetConnection,  
}

impl Copy for Cmd {
    
}

impl Clone for Cmd {
    fn clone(&self) -> Self {
        match self {
            Self::ComSleep => Self::ComSleep,
            Self::ComQuit => Self::ComQuit,
            Self::ComInitDB => Self::ComInitDB,
            Self::ComQuery => Self::ComQuery,
            Self::ComFieldList => Self::ComFieldList,
            Self::ComCreateDb => Self::ComCreateDb,
            Self::ComDropDb => Self::ComDropDb,
            Self::ComRefresh => Self::ComRefresh,
            Self::ComShutdown => Self::ComShutdown,
            Self::ComStatistics => Self::ComStatistics,
            Self::ComProcessInfo => Self::ComProcessInfo,
            Self::ComConnect => Self::ComConnect,
            Self::ComProcessKill => Self::ComProcessKill,
            Self::ComDebug => Self::ComDebug,
            Self::ComPing => Self::ComPing,
            Self::ComTime => Self::ComTime,
            Self::ComDelayedInsert => Self::ComDelayedInsert,
            Self::ComChangeUser => Self::ComChangeUser,
            Self::ComBinlogDump => Self::ComBinlogDump,
            Self::ComTableDump => Self::ComTableDump,
            Self::ComConnectOut => Self::ComConnectOut,
            Self::ComRegisterSlave => Self::ComRegisterSlave,
            Self::ComStmtPrepare => Self::ComStmtPrepare,
            Self::ComStmtExecute => Self::ComStmtExecute,
            Self::ComStmtSendLongData => Self::ComStmtSendLongData,
            Self::ComStmtClose => Self::ComStmtClose,
            Self::ComStmtReset => Self::ComStmtReset,
            Self::ComSetOption => Self::ComSetOption,
            Self::ComStmtFetch => Self::ComStmtFetch,
            Self::ComDaemon => Self::ComDaemon,
            Self::ComBinlogDumpGtid => Self::ComBinlogDumpGtid,
            Self::ComResetConnection => Self::ComResetConnection,
        }
    }
}


impl std::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::ComSleep  => write!(f, "ComSleep"),
            Self::ComQuit => write!(f, "ComQuit"),
            Self::ComInitDB => write!(f, "ComInitDB"),
            Self::ComQuery => write!(f, "ComQuery"),
            Self::ComFieldList => write!(f, "ComFieldList"),
            Self::ComCreateDb => write!(f, "ComCreateDb"),
            Self::ComDropDb => write!(f, "ComDropDb"),
            Self::ComRefresh => write!(f, "ComRefresh"),
            Self::ComShutdown => write!(f, "ComShutdown"),
            Self::ComStatistics => write!(f, "ComStatistics"),
            Self::ComProcessInfo => write!(f, "ComProcessInfo"),
            Self::ComConnect => write!(f, "ComConnect"),
            Self::ComProcessKill => write!(f, "ComProcessKill"),
            Self::ComDebug => write!(f, "ComDebug"),
            Self::ComPing => write!(f, "ComPing"),
            Self::ComTime => write!(f, "ComTime"),
            Self::ComDelayedInsert => write!(f, "ComDelayedInsert"),
            Self::ComChangeUser => write!(f, "ComChangeUser"),
            Self::ComBinlogDump => write!(f, "ComBinlogDump"),
            Self::ComTableDump => write!(f, "ComTableDump"),
            Self::ComConnectOut => write!(f, "ComConnectOut"),
            Self::ComRegisterSlave => write!(f, "ComRegisterSlave"),
            Self::ComStmtPrepare => write!(f, "ComStmtPrepare"),
            Self::ComStmtExecute => write!(f, "ComStmtExecute"),
            Self::ComStmtSendLongData => write!(f, "ComStmtSendLongData"),
            Self::ComStmtClose => write!(f, "ComStmtClose"),
            Self::ComStmtReset => write!(f, "ComStmtReset"),
            Self::ComSetOption => write!(f, "ComSetOption"),
            Self::ComStmtFetch => write!(f, "ComStmtFetch"),
            Self::ComDaemon => write!(f, "ComDaemon"),
            Self::ComBinlogDumpGtid => write!(f, "ComBinlogDumpGtid"),
            Self::ComResetConnection => write!(f, "ComResetConnection"),
        }
    }
}

impl Default for Cmd {
    fn default() -> Self {
        Self::ComSleep
    }
}



impl MysqlParser {
    pub fn  parse_mysql(b: &[u8]) ->Result<MysqlHdr,DumpErr> {
        let mut mysql_hdr  = MysqlHdr{data:Vec::new(),cmd:Default::default()};
        if b.len() == 0 {
            return Err(DumpErr { info: String::from("mysql len stort") });
        }

        match Self::get_cmd(b[4]) {
            Ok(cmd) => {
                let byt = &b[5..];
                mysql_hdr.data = byt.to_vec();       
                mysql_hdr.cmd = Some(cmd);
            }
            Err(dump_err) => {
                return Err(DumpErr { info: dump_err.info });
            }
        }
    
        return Ok(mysql_hdr);
    }

    fn get_cmd(cmd: u8) ->Result<Cmd,DumpErr> {
        if Some(cmd) == Some(Cmd::ComStmtExecute as u8){
            return Ok(Cmd::ComStmtExecute);
        }
        
        if  Some(cmd) == Some(Cmd::ComQuery as u8) {
            return Ok(Cmd::ComQuery);
        }
        
        if Some(cmd) == Some(Cmd::ComCreateDb as u8) {
                return Ok(Cmd::ComCreateDb);
        }

        if Some(cmd) == Some(Cmd::ComStmtPrepare as u8){
            return  Ok(Cmd::ComStmtPrepare);
        } 

        if Some(cmd) == Some(Cmd::ComConnect as u8){
            return  Ok(Cmd::ComConnect);
        }

        if  Some(cmd) == Some(Cmd::ComRegisterSlave as u8) {
            return  Ok(Cmd::ComRegisterSlave);
        }
        
        if Some(cmd) == Some(Cmd::ComInitDB as u8){
            return  Ok(Cmd::ComInitDB);
        }

        return Err(DumpErr { info: String::from("") });
    }
}
