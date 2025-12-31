use parse_cargo_toml::get_package_info;
use chrono::NaiveDate;
use proc_macro::TokenStream;
use quote::quote;
use semver::Version;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn deprecated_until(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析参数，格式如 "1.2.3 2025-12-31"（SemVer + 空格 + yyyy-mm-dd）
    let attr_str = attr.to_string();

    // 移除引号
    let attr_str = attr_str.trim_matches('"').to_string();

    // 按空格分割参数
    let parts: Vec<&str> = attr_str.split_whitespace().collect();
    if parts.len() != 2 {
        panic!("无效的格式：应为 '主版本号.次版本号.修订号 yyyy-mm-dd'，例如：'1.2.3 2025-12-31'");
    }

    let until_semver_str = parts[0];
    let until_date_str = parts[1];

    // 解析 until 的版本号和日期
    let until_version = Version::parse(until_semver_str).expect("无效的版本号格式");
    let until_date = NaiveDate::parse_from_str(until_date_str, "%Y-%m-%d")
        .expect("无效的日期格式，请使用 yyyy-mm-dd 格式");

    // 从 Cargo.toml 获取当前包的版本和更新日期
    let (current_version, current_date) = get_package_info().expect("无法从 Cargo.toml 读取包信息");

    let input = parse_macro_input!(item as Item);

    // 版本比较逻辑：先比较版本号（主版本 > 次版本 > 修订号），如果相等则比较日期
    let had_deprecated = if current_version > until_version {
        true
    } else if current_version == until_version {
        current_date > until_date
    } else {
        false
    };

    let will_deprecated = if !had_deprecated {
        if current_version.major == until_version.major {
            // 副版本号比期限小1或相等，且未达到禁用条件
            current_version.minor == until_version.minor
                || current_version.minor == until_version.minor - 1
        } else {
            false
        }
    } else {
        false // 已经禁用了就不算"即将禁用"
    };

    if had_deprecated {
        // 如果当前版本 > until 版本，则应用 #[deprecated] 属性
        let note = format!(
            "\n此功能在 {} 版本（{}）之后已被弃用，请尽快更新\n（当前版本为 {} 版本（{}）",
            until_semver_str, until_date_str, current_version, current_date
        );

        let deprecated_item = quote! {
            #[deprecated(note = #note)]
            #input
        };

        deprecated_item.into()
    } else if will_deprecated {
        // 如果当前版本 > until 版本，则应用 #[deprecated] 属性
        let note = format!(
            "\n此功能在 {} 版本（{}）之后即将被弃用，请尽快更新\n（当前版本为 {} 版本（{}）",
            until_semver_str, until_date_str, current_version, current_date
        );

        let deprecated_item = quote! {
            #[deprecated(note = #note)]
            #input
        };
        deprecated_item.into()
    } else {
        // 否则，不应用弃用属性
        let normal_item = quote! {
            #input
        };
        normal_item.into()
    }
}
