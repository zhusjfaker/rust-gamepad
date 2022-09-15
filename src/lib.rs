extern crate hidapi;

#[cfg(test)]
mod tests {
    use super::*;
    use hidapi::HidApi;

    #[test]
    fn gamepad_device() {
        println!("Printing all available hid devices:");

        match HidApi::new() {
            Ok(api) => {
                for device in api.device_list() {
                    println!("name-> {:#?}", device.manufacturer_string().unwrap());
//                     println!("origin-> \n
// vendor_id-> {:04x} \n
// product_id->{:04x} \n
// name->{:#?}",
//                              device.vendor_id(),
//                              device.product_id(),
//                              device.manufacturer_string().unwrap()
//                     );
                }
                println!("device number is {:#?}", api.device_list().count())
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
