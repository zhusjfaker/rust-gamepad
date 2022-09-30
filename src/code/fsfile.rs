use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{LineWriter, Write};
use std::path::Path;

/**
 * 计算项目的全局目录
 */
pub fn path_resolve(path: String) -> String {
    let work_cwd = env!("CARGO_MANIFEST_DIR");
    let os_work_cwd = OsString::from(work_cwd);
    return Path::new(&os_work_cwd)
        .join(path)
        .into_os_string()
        .into_string()
        .unwrap();
}

/**
 * 打开txt文件
 */
pub fn safe_open_file(path: String) -> Result<LineWriter<File>, String> {
    let filepath = Path::new(path.as_str());
    let file = if !filepath.exists() {
        match File::create(filepath) {
            Ok(file) => file,
            Err(err) => {
                return Err(err.to_string());
            }
        }
    } else {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(filepath)
            .unwrap()
    };
    let file = LineWriter::new(file);
    Ok(file)
}

pub trait WriteLine {
    fn writeline(&mut self, content: &str);
}

impl WriteLine for LineWriter<File> {
    fn writeline(&mut self, content: &str) {
        self.write_all(content.as_bytes()).unwrap();
        self.write_all(b"\n").unwrap();
    }
}
