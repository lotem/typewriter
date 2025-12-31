use lazy_regex::{ regex, Regex };
use lazy_static::lazy_static;

use crate::definition::{ 击键方式, 转写定义, 输入方案定义, 键位映射定义 };
use crate::gear::layout::拉丁字母键盘布局;
use crate::key_code::KeyCode;
use crate::spelling_algebra::拼写运算;
use crate::转写;
use crate::默认映射;

const 字母表: &[键位映射定义] = &[
    默认映射!(A),
    默认映射!(B),
    默认映射!(C),
    默认映射!(D),
    默认映射!(E),
    默认映射!(F),
    默认映射!(G),
    默认映射!(H),
    默认映射!(I),
    默认映射!(J),
    默认映射!(K),
    默认映射!(L),
    默认映射!(M),
    默认映射!(N),
    默认映射!(O),
    默认映射!(P),
    默认映射!(Q),
    默认映射!(R),
    默认映射!(S),
    默认映射!(T),
    默认映射!(U),
    默认映射!(V),
    默认映射!(W),
    默认映射!(X),
    默认映射!(Y),
    默认映射!(Z),
    键位映射定义 {
        输入码: "␣",
        键码: KeyCode::Space,
    },
    键位映射定义 {
        输入码: "'",
        键码: KeyCode::Quote,
    },
    键位映射定义 {
        输入码: "-",
        键码: KeyCode::Minus,
    },
];

lazy_static! {
    static ref 字母转键位: Box<[拼写运算<'static>]> = Box::new([
        转写!("abcdefghijklmnopqrstuvwxyz ", "ABCDEFGHIJKLMNOPQRSTUVWXYZ␣"),
    ]);
    static ref 貌似拉丁: Box<[&'static Regex]> = Box::new([regex!("^([-A-Za-z '])+$").deref()]);
}

pub fn 输入方案() -> 输入方案定义<'static> {
    输入方案定义 {
        名称: "拉丁字母",
        布局: &拉丁字母键盘布局,
        指法: 击键方式::连击,
        键位映射: 字母表,
        转写: 转写定义 {
            编码预览: &[],
            键位提示: &[],
            输入棱镜: &[],
            词库棱镜: &字母转键位,
            拼式验证规则: &貌似拉丁,
        },
    }
}
