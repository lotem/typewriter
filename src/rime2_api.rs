use custom_attribute_macro::deprecated_until;
use parse_cargo_toml::get_package_info;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref CURRENT_VERSION: Mutex<String> = Mutex::new(String::new());
}

/// **Brief** 获取版本信息API
///
/// **Description** 从Cargo.toml读取版本号和更新日期，返回格式化的版本字符串
///
/// **Return** *const c_char C风格的字符串指针，包含版本信息
///
/// **Safety** 调用者需要负责释放返回的字符串内存
#[deprecated_until("0.0.0 2025-12-31")]
#[no_mangle]
pub extern "C" fn Rime2_API_getVersion() -> *const c_char {
    match get_package_info() {
        Ok((version, date)) => {
            let version_str = format!("Ver{}（{}）", version, date);
            let c_string = CString::new(version_str).unwrap();
            let ptr = c_string.into_raw();

            // 存储到全局变量，确保内存不被释放
            let current_version = CURRENT_VERSION.lock().unwrap();
            // 先释放旧的字符串内存
            if !current_version.is_empty() {
                unsafe {
                    let old_c_string = CString::from_raw(ptr);
                    drop(old_c_string);
                }
            }

            ptr
        }
        Err(e) => {
            let error_msg = format!("Error: {}", e);
            let c_string = CString::new(error_msg.clone()).unwrap();
            let ptr = c_string.into_raw();

            let mut current_version = CURRENT_VERSION.lock().unwrap();
            *current_version = error_msg;

            ptr
        }
    }
}

/// **Brief** 释放版本字符串内存
///
/// **Description** 释放通过rime2_API_getVersion获取的字符串内存
///
/// **Param** ptr *const c_char 需要释放的字符串指针
#[no_mangle]
pub extern "C" fn rime2_API_freeVersionString(ptr: *const c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr as *mut c_char);
        }
    }
}
