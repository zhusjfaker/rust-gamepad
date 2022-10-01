use std::io::Write;

use crate::code::fsfile::{path_resolve, safe_open_file, WriteLine};
use hidapi::HidDevice;

pub fn gamepad_reading(device: HidDevice) {
    let mut old_content = "".to_string();
    println!("starting to read handle byte data...");
    loop {
        let mut buf = [0u8; 8];
        let res = device.read_timeout(&mut buf[..], 1000).unwrap();
        let content = serde_json::to_string(&buf[0..res]).unwrap();
        if content != old_content {
            let record_txt_file = path_resolve("record.txt".to_string());
            let mut file = safe_open_file(record_txt_file).unwrap();
            file.writeline(content.as_str());
            file.flush().unwrap();
            old_content = content;
        }
    }
}
