use lazy_regex::{ regex, Regex };
use lazy_static::lazy_static;

use crate::definition::{ 击键方式, 转写定义, 输入方案定义, 键位映射定义 };
use crate::gear::layout::{ 上档盘面, 基本盘面, 大写字母盘面 };
use crate::gear::layout::{ 盘面定义, 盘面序号, 键盘布局, 键盘配列, 键面刻印 };
use crate::key_code::KeyCode;
use crate::spelling_algebra::拼写运算;
use crate::{ 变换, 消除, 转写 };
use crate::{ 盘面, 键面 };
use crate::键位映射;

const 宫保拼音盘面: 盘面定义<'static> =
    盘面![
    [ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ],
    [ 空 {中:C, 上:_, 下:_, 左:_, 右:"ĉ"} {中:L, 上:_, 下:_, 左:"ĉ", 右:"n"} {中:D, 上:_, 下:_, 左:"n", 右:_} T 空 U {中:R, 上:"er", 下:_, 左:"-i", 右:"-u"} O 空 _ _ ],
    [ 空 {中:S, 上:_, 下:_, 左:_, 右:"ŝ"} {中:H, 上:"x", 下:_, 左:"ŝ", 右:"r"} {中:G, 上:_, 下:_, 左:"r", 右:"j"} {中:K, 上:_, 下:_, 左:_, 右:"q"} 空 I {中:N, 上:_, 下:_, 左:"-n", 右:"-ŋ"} {中:E, 上:_, 下:_, 左:"-ŋ", 右:_} _ _ _ ],
    [ 空 {中:Z, 上:_, 下:_, 左:_, 右:"ẑ"} {中:F, 上:_, 下:_, 左:"ẑ", 右:"m"} {中:B, 上:_, 下:_, 左:"m", 右:_} P 空 Ü _ _ _ ],
    [ A _ A ]
];

const 宫保拼音键盘布局: 键盘布局 = 键盘布局 {
    盘面: &[基本盘面, 上档盘面, 大写字母盘面, 宫保拼音盘面],
    默认盘面: 盘面序号(4),
    默认配列: 键盘配列::正交直列,
};

const 并击键位映射: &[键位映射定义] = &[
    键位映射!(A => KeyCode::Space),
    键位映射!(B => KeyCode::V),
    键位映射!(C => KeyCode::W),
    键位映射!(D => KeyCode::R),
    键位映射!(E => KeyCode::L),
    键位映射!(F => KeyCode::C),
    键位映射!(G => KeyCode::F),
    键位映射!(H => KeyCode::D),
    键位映射!(I => KeyCode::J),
    键位映射!(K => KeyCode::G),
    键位映射!(L => KeyCode::E),
    键位映射!(N => KeyCode::K),
    键位映射!(O => KeyCode::O),
    键位映射!(P => KeyCode::B),
    键位映射!(R => KeyCode::I),
    键位映射!(S => KeyCode::S),
    键位映射!(T => KeyCode::T),
    键位映射!(U => KeyCode::U),
    键位映射!(Z => KeyCode::X),
    键位映射!(Ü => KeyCode::M),
];

lazy_static! {
    static ref 并击转拼音: Box<[拼写运算<'static>]> = Box::new([
        // 空格键单击时产生空白
        变换!("^A$", "␣"),

        // 并击声母
        变换!("^ZF", "zh"),
        变换!("^CL", "ch"),
        变换!("^FB", "m"),
        变换!("^LD", "n"),
        变换!("^HG", "r"),
        // 特殊配列键盘用
        变换!("^ZB", "p"),
        变换!("^CD", "t"),
        变换!("^SG", "k"),

        转写!("BPFDTLGKHZCS", "bpfdtlgkhzcs"),

        // 通摄三等精组、泥来并入一等，须在尖团音规则前变换
        // 浓龙踪从松 ⟨niong/liong/ziong/ciong/siong -> nong/long/zong/cong/song⟩
        变换!("^([nlzcs])(IRO|ÜNE)$", "${1}ong"),

        // G,K,H 接 I/Ü 作 ⟨ji/ju, qi/qu, xi/xu⟩
        // 若分尖团，也可用 Z,C,S 与 I/Ü 相拼
        变换!("^[gz](I|Ü)", "j$1"),
        变换!("^[kc](I|Ü)", "q$1"),
        变换!("^[hs](I|Ü)", "x$1"),

        // ⟨er⟩自成音节
        变换!("^R$", "er"),
        // 舌尖元音⟨ï⟩
        变换!("^([zcsr]h?)R?$", "${1}i"),

        变换!("ANE$", "ang"),
        变换!("UARO$", "uang"),
        变换!("IRO$", "iong"),
        变换!("URO$", "ong"),
        变换!("ÜNE$", "iong"),
        变换!("UNE$", "ong"),
        变换!("INE$", "ing"),
        变换!("NE$", "eng"),

        变换!("AN$", "an"),
        变换!("ÜN$", "vn"),
        变换!("UN$", "uen"),
        变换!("IN$", "in"),
        变换!("N$", "en"),

        变换!("IAR$", "iao"),
        变换!("IR$", "iou"),
        变换!("UR$", "uei"),
        变换!("AO$", "ao"),
        变换!("RO$", "ou"),
        变换!("AR$", "ai"),
        变换!("RE?$", "ei"),
        变换!("AE$", "a"),

        转写!("AOEIUÜ", "aoeiuv"),

        // 汉语拼音方案的拼写规则
        变换!("^i(ng?)$", "yi$1"),
        变换!("^i$", "yi"),
        变换!("^i", "y"),
        变换!("^ong$", "weng"),
        变换!("^u$", "wu"),
        变换!("^u", "w"),
        变换!("^v", "yu"),
        变换!("^([jqx])v", "${1}u"),
        // 一些容错
        变换!("^([bpmf])uo$", "${1}o"),
        变换!("^([dtngkhzcsr]h?)o$", "${1}uo"),
        变换!("io$", "iao"),
        变换!("^([nl])uei$", "${1}ei"),
        变换!("^([nl])iong$", "${1}ong"),
        变换!("^([zcsr]h?)i([aoe])", "$1$2"),
        变换!("^([zcsr]h?)i(ng?)$", "${1}e$2"),
        // 拼写规则
        变换!("iou$", "iu"),
        变换!("uei$", "ui"),
        变换!("uen$", "un"),

        // 声母独用时补足缺省韵母
        // ⟨bu, pu, fu⟩
        变换!("^([bpf])$", "${1}u"),
        // ⟨de, te, ne, le, ge, ke, he⟩
        // 特别地，⟨me⟩ 对应常用字「么·么」
        变换!("^([mdtnlgkh])$", "${1}e"),

        // 检查拼音音节，通过检查则追加隔音符号
        变换!("^([bpm])([iu]|a|i?e|o|[ae]i|i?ao|[oi]u|i?an|[ie]n|[ei]ng|ang|ong)$", "$1$2'"),
        变换!("^([fw])(u|a|o|[ae]i|ao|ou|an|en|eng|ang|ong)$", "$1$2'"),
        变换!("^([dt])([iu]|i?a|i?e|uo|[aeu]i|i?ao|[oi]u|[iu]?an|[ue]n|[ei]ng|ang|ong)$", "$1$2'"),
        变换!(
            "^([nl])([iuv]|i?a|[iv]?e|u?o|[aeu]i|i?ao|[oi]u|[iu]?an|[iue]n|[ei]ng|i?ang|ong)$",
            "$1$2'"
        ),
        变换!("^([gkh])(u|u?a|e|uo|u?ai|[ue]i|ao|ou|u?an|[ue]n|eng|u?ang|ong)$", "$1$2'"),
        变换!("^([zcs]h?|r)([iu]|u?a|e|uo|u?ai|[ue]i|ao|ou|u?an|[ue]n|eng|u?ang|ong)$", "$1$2'"),
        变换!("^([jqxy])([iu]|i?a|[iu]?e|o|i?ao|[oi]u|[iu]?an|[iu]n|ing|i?ang|i?ong)$", "$1$2'"),
        变换!("^([aeo]|[ae]i|ao|ou|[ae]ng?|er)$", "$1'"),
        // 消除不构成合法音节的并击组合
        消除!("^[A-Za-z]+$"),
        转写!("v", "ü"),
        // 显示单个音节不需要加隔音符号
        变换!("^(.*)'$", "$1"),
    ]);

    static ref 拼音转并击: Box<[拼写运算<'static>]> = Box::new([
        // 缺省韵母
        变换!("^bu$", "B"),
        变换!("^pu$", "P"),
        变换!("^me$", "FB"),
        变换!("^fu$", "F"),
        变换!("^de$", "D"),
        变换!("^te$", "T"),
        变换!("^ne$", "LD"),
        变换!("^le$", "L"),
        变换!("^ge$", "G"),
        变换!("^ke$", "K"),
        变换!("^he$", "H"),
        变换!("^zhi$", "ZF"),
        变换!("^chi$", "CL"),
        变换!("^shi$", "SH"),
        变换!("^ri$", "HG"),
        变换!("^zi$", "Z"),
        变换!("^ci$", "C"),
        变换!("^si$", "S"),
        变换!("^er$", "R"),
        // 韵母的并击码
        变换!("^a$", "AE"),
        变换!("a$", "A"),
        变换!("ao$", "AO"),
        变换!("o$", "O"),
        变换!("y?ue$", "ÜE"),
        变换!("e$", "E"),
        变换!("ai$", "AR"),
        变换!("^wei$|ui$", "UR"),
        变换!("^ei$", "RE"),
        变换!("^([zcsr]h?)ei$", "${1}RE"),
        变换!("ei$", "R"),
        变换!("^you$|iou$|iu$", "IR"),
        变换!("ou$", "RO"),
        变换!("an$", "AN"),
        变换!("^yin$|in$", "IN"),
        变换!("^yun$|ün$|vn$", "ÜN"),
        变换!("^([jqx])un$", "${1}ÜN"),
        变换!("^wen$|uen$|un$", "UN"),
        变换!("en$", "N"),
        变换!("wang$|uang$", "UARO"),
        变换!("ang$", "ANE"),
        变换!("^ying$|ing$", "INE"),
        变换!("^yong$|iong$", "IRO"),
        变换!("^weng$|ueng$|ong$", "URO"),
        变换!("eng$", "NE"),
        变换!("^([jqx])u", "${1}Ü"),
        变换!("^yu|ü|v", "Ü"),
        变换!("^yi?|ii?", "I"),
        变换!("^wu?|u", "U"),
        // 声母的并击码
        变换!("^b", "B"),
        变换!("^p", "P"),
        变换!("^m", "FB"),
        变换!("^f", "F"),
        变换!("^d", "D"),
        变换!("^t", "T"),
        变换!("^n", "LD"),
        变换!("^l", "L"),
        变换!("^j-?$", "GI"),
        变换!("^q-?$", "KI"),
        变换!("^x-?$", "HI"),
        变换!("^[gj]", "G"),
        变换!("^[kq]", "K"),
        变换!("^[hx]", "H"),
        变换!("^zh", "ZF"),
        变换!("^ch", "CL"),
        变换!("^sh", "SH"),
        变换!("^z", "Z"),
        变换!("^c", "C"),
        变换!("^s", "S"),
        变换!("^r", "HG"),
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
        名称: "宫保拼音",
        布局: &宫保拼音键盘布局,
        指法: 击键方式::并击,
        键位映射: 并击键位映射,
        转写: 转写定义 {
            编码预览: &[],
            键位提示: &[],
            输入棱镜: &并击转拼音,
            词库棱镜: &拼音转并击,
            拼式验证规则: &貌似拼音,
        },
    }
}
