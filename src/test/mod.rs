#[cfg(test)]
mod tests {
    use crate::code::api::init_hidapi;
    use crate::code::find::{active_gamepad, GamePadProduct};
    use crate::code::keyboard::monitor_keyboard;

    #[test]
    fn gamepad_device() {
        let api = init_hidapi().unwrap();
        let res = active_gamepad(api, GamePadProduct::PS5, false);
        if res.is_none() {
            println!("can't find active ps5 gamepad controller !");
        }
        let _device = res.unwrap();

        println!("test end ...")
    }

    #[test]
    fn keyboard_device() {
      monitor_keyboard(); 
    }
}
