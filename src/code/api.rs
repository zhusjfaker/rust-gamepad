use hidapi::HidApi;

///
/// 初始化 hidapi 库
/// 检查 相关 *.dll 配置
///
pub fn init_hidapi() -> Option<HidApi> {
    match HidApi::new() {
        Ok(api) => {
            println!("init hidapi successful!");
            Some(api)
        }
        Err(error) => {
            println!("init hidapi has error -> {:#?}", error);
            println!("Please confirm whether hidapi is installed ......");
            if cfg!(target_os = "windows") {
                println!(r#"windows -> browse https://github.com/libusb/hidapi/releases "#)
            } else if cfg!(target_os = "linux") {
                println!("linux -> brew install hidapi");
            } else if cfg!(target_os = "macos") {
                println!("macos -> brew install hidapi");
            } else {
                println!("not yet support hidapi!");
            }
            None
        }
    }
}
