use std::env;
use std::ffi::{OsString, OsStr};
use std::collections::HashMap;

fn main() {
    let envs: HashMap<OsString, OsString> = env::vars_os().collect();
    if let Some(home) = envs.get(OsStr::new("HOME")) {
        if home != "/home/enebo" {
            println!("HOME: {:?}", home);
        }
    }

    let envs: HashMap<String, String> = env::vars().collect();
    if let Some(home) = envs.get("HOME") {
        if home == "/home/enebo" {
            println!("HOME: {:?}", home);
        }
    }
}
