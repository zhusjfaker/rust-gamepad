use hidapi::HidDevice;
use crate::code::fsfile::{path_resolve, safe_open_file, WriteLine};

pub fn gamepad_reading(device: HidDevice) {
    let mut old_content = "".to_string();
    loop {
        println!("Reading...");
        let mut buf = [0u8; 8];
        let _res = device.read_timeout(&mut buf[..], 1000).unwrap();
        // println!("Read: {:?}", &buf[..res]);
        let content = String::from_utf8(buf.to_vec()).unwrap();
        if content != old_content {
            let record_txt_file = path_resolve("record.txt".to_string());
            let mut file = safe_open_file(record_txt_file).unwrap();
            file.writeline(content.as_str());
            old_content = content;
        }
    }
}