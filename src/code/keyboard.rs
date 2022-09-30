use device_query::{DeviceQuery, DeviceState, Keycode};

///
/// 监听键盘按键
///
pub fn monitor_keyboard() {
    println!("starting monitor keyboard ...");
    let device_state = DeviceState::new();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.len() > 0 && keys[0] == Keycode::F1 {
            println!("monitor keyboard has ended!...");
            break;
        } else if keys.len() > 0 {
            for key in keys {
                println!("keyboard button has pressed {:#?}", key);
            }
        }
    }
}
