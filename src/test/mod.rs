#[cfg(test)]
mod tests {
    use crate::code::api::init_hidapi;
    use crate::code::find::{active_gamepad, GamePadProduct};
    use crate::code::fsfile::{path_resolve, safe_open_file, WriteLine};
    use crate::code::keyboard::monitor_keyboard;
    use crate::code::read::gamepad_reading;
    use std::io::Write;

    #[test]
    fn gamepad_device() {
        let api = init_hidapi().unwrap();
        let res = active_gamepad(api, GamePadProduct::PS5, false);
        if res.is_none() {
            println!("can't find active ps5 gamepad controller !");
        }
        let device = res.unwrap();
        gamepad_reading(device);
        println!("test end ...")
    }

    #[test]
    fn keyboard_device() {
        monitor_keyboard();
    }

    #[test]
    fn test_write_txt() {
        let test_txt_file = path_resolve("test.txt".to_string());
        let mut file = safe_open_file(test_txt_file).unwrap();
        file.writeline("123");
        file.writeline("456");
        file.flush().unwrap();
    }
}
