
use std::process::Command;
use std::str;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Partition {
  pub name: String,
  pub rm: bool,
  pub size: u128,
  pub ro: bool,
  pub mountpoints: Vec<Option<String>>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Device {
  pub name: String,
  pub rm: bool,
  pub size: u128,
  pub ro: bool,
  pub children: Option<Vec<Partition>>
}

#[derive(Serialize, Deserialize)]
pub struct Devices {
  pub blockdevices: Vec<Device>
}

impl Devices { 
  pub fn get(&self, key: usize) -> &Device {
    return &self.blockdevices[key];
  }
}

pub fn get_devices() -> Devices {
  let raw_data = Command::new("lsblk")
    .args(vec!["-J", "-b"])
    .output()
    .expect("Failed to execute lsblk");
  
  let buf:   &[u8] = &raw_data.stdout;
  
  let fdata: &str  = str::from_utf8(&buf).unwrap();

  let data: Devices = serde_json::from_str(
    fdata
  ).unwrap();

  return data;
}
