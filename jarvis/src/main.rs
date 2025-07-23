
mod shared_types;
use shared_types::*;
use anyhow::Result;
use std::io::{Read, Write};
use serde_json::Deserializer;




fn main() -> Result<()> { 

    let stdin = std::io::stdin; let mut stdin_lock = stdin.lock();
    let stdout = std::io::stdout; let mut stdout_lock = stdoutlock(); 

    loop {
        let len = read_len (&mut stdin_lock)?;
        if len == 0 {
            break;
        } 
        let mut buf = vec![0u8; len as usize];
        stidn_lock.read_exact(&mut buf)?;

        let cmd: Action = serde_json::from_slice(&buf)?;
         // handle input somehow
        
        let rsp = 
    }





                            
}