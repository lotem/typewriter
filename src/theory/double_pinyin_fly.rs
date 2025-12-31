use lazy_regex::{ regex, Regex };
use lazy_static::lazy_static;

use crate::definition::{ 击键方式, 转写定义, 输入方案定义, 键位映射定义 };
use crate::gear::layout::{ 上档盘面, 基本盘面, 大写字母盘面 };
use crate::gear::layout::{ 盘面定义, 盘面序号, 键盘布局, 键盘配列, 键面刻印 };
use crate::key_code::KeyCode;
use crate::spelling_algebra::拼写运算;
use crate::{ 变换, 派生, 消除, 缩写, 转写 };
use crate::{ 盘面, 键面 };
use crate::默认映射;

const 小鹤双拼助记盘面: 盘面定义<'static> =
    盘面![
    [ _ _ _ _ _ _ _ _ _ _ ],
    [ {中:Q, 上:_, 下:"iu", 左:_, 右:_} {中:W, 上:_, 下:"ei", 左:_, 右:_} {中:E, 上:_, 下:"e", 左:_, 右:_} {中:R, 上:_, 下:"uan/van", 左:_, 右:_} {中:T, 上:_, 下:"ue/ve", 左:_, 右:_} {中:Y, 上:_, 下:"un/vn", 左:_, 右:_} {中:U, 上:_, 下:"u", 左:_, 右:_} {中:I, 上:_, 下:"i", 左:_, 右:_} {中:O, 上:_, 下:"uo/o", 左:_, 右:_} {中:P, 上:_, 下:"ie", 左:_, 右:_} ],
    [ {中:A, 上:_, 下:"a", 左:_, 右:_} {中:S, 上:_, 下:"ong/iong", 左:_, 右:_} {中:D, 上:_, 下:"ai", 左:_, 右:_} {中:F, 上:_, 下:"en", 左:_, 右:_} {中:G, 上:_, 下:"eng", 左:_, 右:_} {中:H, 上:_, 下:"ang", 左:_, 右:_} {中:J, 上:_, 下:"an", 左:_, 右:_} {中:K, 上:_, 下:"uai/ing", 左:_, 右:_} {中:L, 上:_, 下:"uang/iang", 左:_, 右:_} {中:";", 上:":", 下:_, 左:_, 右:_} ],
    [ {中:Z, 上:_, 下:"ou", 左:_, 右:_} {中:X, 上:_, 下:"ua/ia", 左:_, 右:_} {中:C, 上:_, 下:"ao", 左:_, 右:_} {中:V, 上:_, 下:"ui/v", 左:_, 右:_} {中:B, 上:_, 下:"in", 左:_, 右:_} {中:N, 上:_, 下:"iao", 左:_, 右:_} {中:M, 上:_, 下:"ian", 左:_, 右:_} {中:",", 上:"<", 下:_, 左:_, 右:_} {中:".", 上:">", 下:_, 左:_, 右:_} {中:"/", 上:"?", 下:_, 左:_, 右:_} ],
    [ "␣" _ "␣" ]
];

const 小鹤双拼助记键盘布局: 键盘布局 = 键盘布局 {
    盘面: &[基本盘面, 上档盘面, 大写字母盘面, 小鹤双拼助记盘面],
    默认盘面: 盘面序号(4),
    默认配列: 键盘配列::正交直列,
};

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
    static ref 小鹤转全拼: Box<[拼写运算<'static>]> = Box::new([
        变换!("([BPMFDTNLJQX])N", "${1}iao"),
        变换!("(\\w)G", "${1}eng"),
        变换!("(\\w)Q", "${1}iu"),

        变换!("(\\w)W", "${1}ei"),
        变换!("([DTNLGKHJQXYVIURZCS])R", "${1}uan"),
        变换!("(\\w)T", "${1}ve"),
        变换!("(\\w)Y", "${1}un"),
        变换!("([DTNLGKHVUIRzcs])O", "${1}uo"),
        变换!("(\\w)P", "{$}ie"),
        变换!("([JQX])S", "${1}iong"),
        变换!("(\\w)S", "${1}ong"),
        变换!("(\\w)D", "${1}ai"),
        变换!("(\\w)F", "${1}en"),
        变换!("(\\w)H", "${1}ang"),
        变换!("(\\w)J", "${1}an"),
        变换!("([GKHVUIRZCS])K", "${1}uai"),
        变换!("(\\w)K", "${1}ing"),
        变换!("([JQXNL])L", "${1}iang"),
        变换!("(\\w)L", "${1}uang"),
        变换!("(\\w)Z", "${1}ou"),
        变换!("([GKHVUIRZCS])X", "${1}ua"),
        变换!("(\\w)X", "${1}ia"),
        变换!("(\\w)C", "${1}ao"),
        变换!("([DTGKHVUIRZCS])V", "${1}ui"),
        变换!("(\\w)B", "${1}in"),
        变换!("(\\w)M", "${1}ian"),
        // 变换!("([AOE])\\1(\\w)", "$1$2"),

        变换!("(^|[ ])V", "${1}zh"),
        变换!("(^|[ ])I", "${1}ch"),
        变换!("(^|[ ])U", "${1}sh"),

        变换!("([JQXY])V", "${1}u"),
        变换!("([NL])V", "${1}v"),

        转写!("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz"),
    ]);
    static ref 全拼转小鹤: Box<[拼写运算<'static>]> = Box::new([
        变换!("cong", "CS"),
        变换!("ming", "MK"),
    ]);
    static ref 键位转字母: Box<[拼写运算<'static>]> = Box::new([
        变换!("([BPMFDTNLJQX])N", "$1iao"),
        变换!("(\\w)G", "$1eng"),
        变换!("(\\w)Q", "$1iu"),
        变换!("(\\w)W", "$1ei"),
        变换!("([DTNLGKHJQXYVIURZCS])R", "$1uan"),
        变换!("(\\w)T", "$1ve"),
        变换!("(\\w)Y", "$1un"),
        变换!("([DTNLGKHVUIRzcs])o", "$1uo"),
        变换!("(\\w)P", "$1ie"),
        变换!("([JQX])S", "$1iong"),
        变换!("(\\w)S", "$1ong"),
        变换!("(\\w)D", "$1ai"),
        变换!("(\\w)F", "$1en"),
        变换!("(\\w)H", "$1ang"),
        变换!("(\\w)J", "$1an"),
        变换!("([GKHVUIRZCS])K", "$1uai"),
        变换!("(\\w)K", "$1ing"),
        变换!("([JQXNL])L", "$1iang"),
        变换!("(\\w)L", "$1uang"),
        变换!("(\\w)Z", "$1ou"),
        变换!("([GKHVUIRZCS])X", "$1ua"),
        变换!("(\\w)X", "$1ia"),
        变换!("(\\w)C", "$1ao"),
        变换!("([DTGKHVUIRZCS])V", "$1ui"),
        变换!("(\\w)B", "$1in"),
        变换!("(\\w)M", "$1ian"),
        // 变换!("([AOE])\\1(\\w)", "$1$2"),

        变换!("(^|[ ])V", "$1zh"),
        变换!("(^|[ ])I", "$1ch"),
        变换!("(^|[ ])U", "$1sh"),

        变换!("([JQXY])V", "$1u"),
        变换!("([NL])V", "$1ü"),
    ]);
    static ref 字母转键位: Box<[拼写运算<'static>]> = Box::new([
        消除!("^xx$"),

        派生!("^([jqxy])u$", "$1V"),
        派生!("^([aoe])([ioun])$", "$1$1$2"),

        变换!("^([aoe])(ng)?$", "$1$1$2"),

        变换!("iu$", "Q"),
        变换!("(.)ei$", "$1W"),
        变换!("uan$", "R"),
        变换!("[uv]e$", "T"),
        变换!("un$", "Y"),
        变换!("^sh", "U"),
        变换!("^ch", "I"),
        变换!("^zh", "V"),
        变换!("uo$", "O"),
        变换!("ie$", "P"),
        变换!("i?ong$", "S"),
        变换!("ing$|uai$", "K"),
        变换!("(.)ai$", "$1D"),
        变换!("(.)en$", "$1F"),
        变换!("(.)eng$", "$1G"),
        变换!("[iu]ang$", "L"),
        变换!("(.)ang$", "$1H"),
        变换!("ian$", "M"),
        变换!("(.)an$", "$1J"),
        变换!("(.)ou$", "$1Z"),
        变换!("[iu]a$", "X"),
        变换!("iao$", "N"),
        变换!("(.)ao$", "$1C"),
        变换!("ui$", "V"),
        变换!("in$", "B"),

        转写!("qwrtyuiopsdfghjklzxcvbnm", "QWRTYUIOPSDFGHJKLZXCVBNM"),
        缩写!("^(.).+$", "$1"),
    ]);
    static ref 貌似拼音: Box<[&'static Regex]> = Box::new([
        regex!("^([bpm])([iu]|a|i?e|o|[ae]i|i?ao|[oi]u|i?an|[ie]n|[ei]ng|ang|ong)$").deref(),
        regex!("^([fw])(u|a|o|[ae]i|ao|ou|an|en|eng|ang|ong)$").deref(),
        regex!("^([dt])([iu]|i?a|i?e|uo|[aeu]i|i?ao|[oi]u|[iu]?an|[ue]n|[ei]ng|ang|ong)$").deref(),
        regex!(
            "^([nl])([iuv]|i?a|[iuv]?e|üe?|u?o|[aeu]i|i?ao|[oi]u|[iu]?an|[iue]n|[ei]ng|i?ang|i?ong)$"
        ).deref(),
        regex!("^([gkh])(u|u?a|e|uo|u?ai|[ue]i|ao|ou|u?an|[ue]n|eng|u?ang|ong)$").deref(),
        regex!("^([zcs]h?|r)([iu]|u?a|e|uo|u?ai|[ue]i|ao|ou|u?an|[ue]n|eng|u?ang|ong)$").deref(),
        regex!("^([jqxy])([iu]|i?a|[iu]?e|o|i?ao|[oi]u|[iu]?an|[iu]n|ing|i?ang|i?ong)$").deref(),
        // 尖音，演示指法用。其中韵母 i 双写
        regex!("^([zcs])(ii|[iv]e?|üe?|i?ao|iu|[iv]a?n|üa?n|ia?ng|iong)$").deref(),
        regex!("^([aeo]|[ae]i|ao|ou|[ae]ng?|er)$").deref(),
        // 声母
        regex!("^([bpmfdtnlgkhjqxr]|[zcs]h?)-?$").deref(),
        // 非音节形式的韵母
        regex!("^([yw])-?$").deref(),
        regex!(
            "^-?([iuv]|[iu]?[ao]|[iuv]?e|üe?|u?[ae]i|ui|i?ao|i?ou|iu|[iuv]?an|üa?n|[iuv]n|u?en|[iu]?ang|ing|u?eng|i?ong)?$"
        ).deref(),
    ]);
}

pub fn 输入方案() -> 输入方案定义<'static> {
    输入方案定义 {
        名称: "小鹤双拼",
        布局: &小鹤双拼助记键盘布局,
        指法: 击键方式 /* 连 */::并击,
        键位映射: 字母表,
        转写: 转写定义 {
            编码预览: &[] /* 小鹤转全拼 */,
            键位提示: &字母转键位 /* []全拼转小鹤 */,
            输入棱镜: &小鹤转全拼 /* [] */ /* 键位转字母 */,
            词库棱镜: &[] /* 字母转键位 */,
            拼式验证规则: &貌似拼音,
        },
    }
}
