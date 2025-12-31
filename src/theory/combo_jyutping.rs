use lazy_regex::{ regex, Regex };
use lazy_static::lazy_static;

use crate::definition::{ 击键方式, 转写定义, 输入方案定义, 键位映射定义 };
use crate::gear::layout::{ 上档盘面, 基本盘面, 大写字母盘面 };
use crate::gear::layout::{ 盘面定义, 盘面序号, 键盘布局, 键盘配列, 键面刻印 };
use crate::key_code::KeyCode;
use crate::spelling_algebra::拼写运算;
use crate::{ 变换, 转写 };
use crate::{ 盘面, 键面 };
use crate::键位映射;

const 宫保粤拼盘面: 盘面定义<'static> =
    盘面![
    [ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ],
    [ 空 {中:c, 上:_, 下:_, 左:_, 右:j} {中:l, 上:_, 下:_, 左:_, 右:_} {中:d, 上:_, 下:_, 左:n, 右:_} t o {中:u, 上:_, 下:_, 左:_, 右:ou} {中:"˗u", 上:_, 下:_, 左:_, 右:_} {中:"˗t", 上:_, 下:_, 左:"˗n", 右:_} 空 _ _ ],
    [ 空 {中:s, 上:_, 下:_, 左:_, 右:w} {中:h, 上:_, 下:_, 左:_, 右:_} {中:g, 上:_, 下:_, 左:ng, 右:_} k e {中:i, 上:_, 下:_, 左:_, 右:ei} {中:"˗i", 上:_, 下:_, 左:_, 右:_} {中:"˗k", 上:_, 下:_, 左:"˗ng", 右:_} 空 _ ],
    [ 空 {中:z, 上:_, 下:_, 左:_, 右:"'"} {中:f, 上:_, 下:_, 左:_, 右:_} {中:b, 上:_, 下:_, 左:m, 右:_} p {中:oe, 上:_, 下:eo, 左:_, 右:_} yu {中:"m", 上:_, 下:_, 左:_, 右:_} {中:"˗p", 上:_, 下:_, 左:"˗m", 右:_} 空 ],
    [ aa _ aa ]
];

const 宫保粤拼布局: 键盘布局 = 键盘布局 {
    盘面: &[基本盘面, 上档盘面, 大写字母盘面, 宫保粤拼盘面],
    默认盘面: 盘面序号(4),
    默认配列: 键盘配列::正交直列,
};

const 并击键位映射: &[键位映射定义] = &[
    键位映射!(A => KeyCode::Space),
    键位映射!(B => KeyCode::V),
    键位映射!(C => KeyCode::W),
    键位映射!(D => KeyCode::R),
    键位映射!(E => KeyCode::H),
    键位映射!(F => KeyCode::C),
    键位映射!(G => KeyCode::F),
    键位映射!(H => KeyCode::D),
    键位映射!(I => KeyCode::J),
    键位映射!(J => KeyCode::K),
    键位映射!(K => KeyCode::G),
    键位映射!(L => KeyCode::E),
    键位映射!(M => KeyCode::Comma),
    键位映射!(O => KeyCode::Y),
    键位映射!(P => KeyCode::B),
    键位映射!(Q => KeyCode::L),
    键位映射!(R => KeyCode::O),
    键位映射!(S => KeyCode::S),
    键位映射!(T => KeyCode::T),
    键位映射!(U => KeyCode::U),
    键位映射!(V => KeyCode::N),
    键位映射!(W => KeyCode::I),
    键位映射!(X => KeyCode::Dot),
    键位映射!(Y => KeyCode::M),
    键位映射!(Z => KeyCode::X),
];

lazy_static! {
    static ref 并击码表示: Box<[拼写运算<'static>]> = Box::new([
        变换!("^FBM$", "m"),
        变换!("^HGM$", "ng"),
        变换!("FB", "m"),
        变换!("LD", "n"),
        变换!("HG", "ng"),
        变换!("SG", "gw"),
        变换!("SK", "kw"),
        变换!("SH", "w"),
        变换!("CL", "j"),
        变换!("ZF", ""),
        变换!("AI", "E"),
        变换!("AU", "O"),
        变换!("AY", "V"),
        变换!("A", "aa"),
        变换!("Y", "yu"),
        变换!("V(J|W?R)$", "eo${1}"),
        变换!("V", "oe"),
        变换!("JQ", "-ng"),
        变换!("WR", "-n"),
        变换!("MX", "-m"),
        变换!("Q$", "-k"),
        变换!("R$", "-t"),
        变换!("X$", "-p"),
        变换!("IJ", "ei"),
        变换!("UW", "ou"),
        变换!("J$", "-i"),
        变换!("W$", "-u"),
        转写!("SCZHLFGDBKTPEOIUM", "sczhlfgdbktpeoium"),
        变换!("^([bpmfdtnlgkhzcsjw]|ng|[gk]w*)([-aeiouy])", "${1} ${2}"),
    ]);
    static ref 并击码键位: Box<[拼写运算<'static>]> = Box::new([
        变换!("^m", "FB"),
        变换!("^ng", "HG"),
        变换!("^n", "LD"),
        变换!("^gw", "SG"),
        变换!("^kw", "SK"),
        变换!("^w([^u])", "SH${1}"),
        变换!("^j([^iy])", "CL${1}"),
        变换!("-n$", "WR"),
        变换!("-t$", "R"),
        变换!("-ng$", "JQ"),
        变换!("-k$", "Q"),
        变换!("-m$", "MX"),
        变换!("-p$", "X"),
        变换!("ei$", "IJ"),
        变换!("-i$", "J"),
        变换!("ou$", "UW"),
        变换!("-u$", "W"),
        变换!("aa", "A"),
        变换!("oe|eo", "V"),
        变换!("^j([iy])", "${1}"),
        变换!("^w(u)", "${1}"),
        变换!("^j", "CL"),
        变换!("^w", "SH"),
        变换!("yu", "Y"),
        转写!("sczhlfgdbktpeoiu", "SCZHLFGDBKTPEOIU"),
    ]);

    static ref 并击转粤拼: Box<[拼写运算<'static>]> = Box::new([
        变换!("^FBM$", "m"),
        变换!("^HGM$", "ng"),
        变换!("FB", "m"),
        变换!("LD", "n"),
        变换!("HG", "ng"),
        变换!("SG", "gw"),
        变换!("SK", "kw"),
        变换!("SH", "w"),
        变换!("CL", "j"),
        变换!("ZF", ""),
        变换!("AI", "E"),
        变换!("AU", "O"),
        变换!("AY", "V"),
        变换!("A", "aa"),
        变换!("Y", "yu"),
        变换!("V(J|W?R)$", "eo${1}"),
        变换!("V", "oe"),
        变换!("JQ", "-ng"),
        变换!("WR", "-n"),
        变换!("MX", "-m"),
        变换!("Q$", "-k"),
        变换!("R$", "-t"),
        变换!("X$", "-p"),
        变换!("IJ", "ei"),
        变换!("UW", "ou"),
        变换!("J$", "-i"),
        变换!("W$", "-u"),
        转写!("SCZHLFGDBKTPEOIUM", "sczhlfgdbktpeoium"),
        变换!("([aeiou])-([iumptk]|ng?)$", "${1}${2}"),
        变换!("-", "a"),
        变换!(" ", ""),
        变换!("^([iy])", "j${1}"),
        变换!("^(u)", "w${1}"),
    ]);
    static ref 粤拼转并击: Box<[拼写运算<'static>]> = Box::new([
        变换!("^m", "FB"),
        变换!("^ng", "HG"),
        变换!("^n", "LD"),
        变换!("^gw", "SG"),
        变换!("^kw", "SK"),
        变换!("^w([^u])", "SH${1}"),
        变换!("^j([^iy])", "CL${1}"),
        变换!("([aeiou])n$", "${1}WR"),
        变换!("([aeoiu])t$", "${1}R"),
        变换!("([aeoiu])ng$", "${1}JQ"),
        变换!("([aeoiu])k$", "${1}Q"),
        变换!("([aeoiu])m$", "${1}MX"),
        变换!("([aeoiu])p$", "${1}X"),
        变换!("ei$", "IJ"),
        变换!("([aou])i$", "${1}J"),
        变换!("ou$", "UW"),
        变换!("([aei])u$", "${1}W"),
        变换!("aa", "A"),
        变换!("oe|eo", "V"),
        变换!("^j([iy])", "${1}"),
        变换!("^w(u)", "${1}"),
        变换!("^j", "CL"),
        变换!("^w", "SH"),
        变换!("yu", "Y"),
        变换!("a", ""),
        转写!("sczhlfgdbktpeoiu", "SCZHLFGDBKTPEOIU"),
    ]);
}

lazy_static! {
    static ref 貌似粤拼: Box<[&'static Regex]> = Box::new([
        regex!("^[bpmfdtnlgkhzcsjwaoey][a-z]*$").deref(),
    ]);
}

pub fn 输入方案() -> 输入方案定义<'static> {
    输入方案定义 {
        名称: "宫保粤拼",
        布局: &宫保粤拼布局,
        指法: 击键方式::并击,
        键位映射: 并击键位映射,
        转写: 转写定义 {
            编码预览: &并击码表示,
            键位提示: &并击码键位,
            输入棱镜: &并击转粤拼,
            词库棱镜: &粤拼转并击,
            拼式验证规则: &貌似粤拼,
        },
    }
}
