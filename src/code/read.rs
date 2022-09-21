use hidapi::HidDevice;

pub fn gamepad_reading(device: HidDevice) {
    loop {
        println!("Reading...");
        let mut buf = [0u8; 8];
        let res = device.read_timeout(&mut buf[..], 1000).unwrap();
        println!("Read: {:?}", &buf[..res]);
        
    }
}