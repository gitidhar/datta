use std::process::Command; 
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


pub fn launch_an_app() {

    Command::new("open")
    .arg("-a")
    .arg("Spotify")
    .spawn()
    .expect("failed to open Spotify");

} 

// enumeration for what a command can actuall
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Comamnd {
    SetValue {selector: String, value: String},
    SelectOption {selector: String, needle: String, by: SelectBy},
    Click {selector: String}, 
    WaitFor {selctor: String, timeout_ms: u64},
    RunScript {code: String},
} 

