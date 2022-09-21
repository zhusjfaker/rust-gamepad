use hidapi::{HidApi, HidDevice};

pub enum GamePadProduct {
    PS5,
}

impl GamePadProduct {
    fn value(&self) -> String {
        match *self {
            GamePadProduct::PS5 => "DualSense Wireless Controller".to_string(),
        }
    }
}

///
/// 激活当前_游戏手柄
/// 未找到 PS5 | XSS (not yet support)
/// 返回为 空
///
pub fn active_gamepad(api: HidApi, gamepad_type: GamePadProduct, info: bool) -> Option<HidDevice> {
    println!("There are {:#?} currently connected devices! ......", api.device_list().count());
    for device in api.device_list() {
        let origin_vendor_id = device.vendor_id();
        let origin_product_id = device.product_id();
        let device_active = match api.open(origin_vendor_id, origin_product_id) {
            Ok(active_device) => {
                Some(active_device)
            }
            Err(_error) => {
                if info {
                    println!("vid -> {:#?} pid->{:#?} device has error has skipped", origin_vendor_id, origin_product_id);
                }
                None
            }
        };
        if device_active.is_none() {
            continue;
        }
        let hid_device = device_active.unwrap();
        let manufacturer = match hid_device.get_manufacturer_string() {
            Ok(res) => {
                res.unwrap_or("".to_string())
            }
            Err(_ex) => {
                "".to_string()
            }
        };
        let product = match hid_device.get_product_string() {
            Ok(res) => {
                res.unwrap_or("".to_string())
            }
            Err(_ex) => {
                "".to_string()
            }
        };
        if info {
            println!("manufacturer -> {:#?}", manufacturer);
            println!("product -> {:#?}", product);
        }
        if product == gamepad_type.value() {
            return Some(hid_device);
        }
    }
    None
}