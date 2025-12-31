use chrono::NaiveDate;
use semver::Version;
use std::fs;
// use custom_attribute_macro::deprecated_until;

pub fn get_package_info() -> Result<(Version, NaiveDate), Box<dyn std::error::Error>> {
    // 获取当前工作目录（宏被调用的项目目录）
    let cargo_toml_path = find_cargo_toml()?;

    // 读取 Cargo.toml 内容
    let toml_content = fs::read_to_string(&cargo_toml_path)?;

    // 解析 TOML
    let toml_value: toml::Value = toml::from_str(&toml_content)?;

    // 获取 package.version
    let version_str = toml_value
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .ok_or("在 Cargo.toml 中未找到 package.version 字段")?;

    let current_version = Version::parse(version_str)?;

    // 获取 package.metadata.update_date
    let date_str = toml_value
        .get("package")
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("update_date"))
        .and_then(|d| d.as_str())
        .ok_or("在 Cargo.toml 中未找到 package.metadata.update_date 字段")?;

    let current_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;

    Ok((current_version, current_date))
}

/// 查找 Cargo.toml 文件
fn find_cargo_toml() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut current_dir = std::env::current_dir()?;

    loop {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        if cargo_toml_path.exists() {
            return Ok(cargo_toml_path);
        }

        // 向上级目录查找
        if !current_dir.pop() {
            break;
        }
    }

    Err("在当前目录或父目录中未找到 Cargo.toml 文件".into())
}
