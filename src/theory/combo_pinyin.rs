use lazy_regex::{regex, Regex};
use lazy_static::lazy_static;

use crate::definition::{
    碼表格式, 觸鍵方式, 輸入方案定義, 轉寫法定義, 邊界判定規則, 鍵位定義
};
use crate::gear::layout::{
    上檔盤面, 基本盤面, 大寫字母盤面, 盤面定義, 盤面選擇碼, 配列, 鍵盤佈局, 鍵面刻印,
};
use crate::gear::theory::輸入方案環境;
use crate::key_code::KeyCode;
use crate::spelling_algebra::拼寫運算;
use crate::{消除, 盤面, 變換, 轉寫, 鍵面};

macro_rules! 鍵位 {
    ($輸入碼: ident => $鍵碼: path) => {
        鍵位定義 {
            輸入碼: stringify!($輸入碼),
            盤面: 盤面選擇碼(0),
            鍵碼: $鍵碼,
        }
    };
}

const 並擊鍵序: &[鍵位定義] = &[
    鍵位!(S => KeyCode::S),
    鍵位!(C => KeyCode::W),
    鍵位!(Z => KeyCode::X),
    鍵位!(H => KeyCode::D),
    鍵位!(L => KeyCode::E),
    鍵位!(F => KeyCode::C),
    鍵位!(G => KeyCode::F),
    鍵位!(D => KeyCode::R),
    鍵位!(B => KeyCode::V),
    鍵位!(K => KeyCode::G),
    鍵位!(T => KeyCode::T),
    鍵位!(P => KeyCode::B),
    鍵位!(I => KeyCode::J),
    鍵位!(U => KeyCode::U),
    鍵位!(Ü => KeyCode::M),
    鍵位!(A => KeyCode::Space),
    鍵位!(N => KeyCode::K),
    鍵位!(R => KeyCode::I),
    鍵位!(E => KeyCode::L),
    鍵位!(O => KeyCode::O),
];

lazy_static! {
    static ref 並擊轉拼音: Box<[拼寫運算::<'static>]> = Box::new([
        // 空格鍵單擊時產生空白
        變換!("^A$", "␣"),

        // 並擊聲母
        變換!("^ZF", "zh"),
        變換!("^CL", "ch"),
        變換!("^FB", "m"),
        變換!("^LD", "n"),
        變換!("^HG", "r"),
        // 特殊配列鍵盤用
        變換!("^ZB", "p"),
        變換!("^CD", "t"),
        變換!("^SG", "k"),

        轉寫!("BPFDTLGKHZCS", "bpfdtlgkhzcs"),

        // 通摄三等精組、泥來併入一等，須在尖團音規則前變換
        // 濃龍蹤從松 ⟨niong/liong/ziong/ciong/siong -> nong/long/zong/cong/song⟩
        變換!("^([nlzcs])(IRO|ÜNE)$", "${1}ong"),

        // G,K,H 接 I/Ü 作 ⟨ji/ju, qi/qu, xi/xu⟩
        // 若分尖團，也可用 Z,C,S 與 I/Ü 相拼
        變換!("^[gz](I|Ü)", "j$1"),
        變換!("^[kc](I|Ü)", "q$1"),
        變換!("^[hs](I|Ü)", "x$1"),

        // ⟨er⟩自成音節
        變換!("^R$", "er"),
        // 舌尖元音⟨ï⟩
        變換!("^([zcsr]h?)R?$", "${1}i"),

        變換!("ANE$", "ang"),
        變換!("UARO$", "uang"),
        變換!("IRO$", "iong"),
        變換!("URO$", "ong"),
        變換!("ÜNE$", "iong"),
        變換!("UNE$", "ong"),
        變換!("INE$", "ing"),
        變換!("NE$", "eng"),

        變換!("AN$", "an"),
        變換!("ÜN$", "vn"),
        變換!("UN$", "uen"),
        變換!("IN$", "in"),
        變換!("N$", "en"),

        變換!("IAR$", "iao"),
        變換!("IR$", "iou"),
        變換!("UR$", "uei"),
        變換!("AO$", "ao"),
        變換!("RO$", "ou"),
        變換!("AR$", "ai"),
        變換!("RE?$", "ei"),
        變換!("AE$", "a"),

        轉寫!("AOEIUÜ", "aoeiuv"),

        // 漢語拼音方案的拼寫規則
        變換!("^i(ng?)$", "yi$1"),
        變換!("^i$", "yi"),
        變換!("^i", "y"),
        變換!("^ong$", "weng"),
        變換!("^u$", "wu"),
        變換!("^u", "w"),
        變換!("^v", "yu"),
        變換!("^([jqx])v", "${1}u"),
        // 一些容錯
        變換!("^([bpmf])uo$", "${1}o"),
        變換!("^([dtngkhzcsr]h?)o$", "${1}uo"),
        變換!("io$", "iao"),
        變換!("^([nl])uei$", "${1}ei"),
        變換!("^([nl])iong$", "${1}ong"),
        變換!("^([zcsr]h?)i([aoe])", "$1$2"),
        變換!("^([zcsr]h?)i(ng?)$", "${1}e$2"),
        // 拼寫規則
        變換!("iou$", "iu"),
        變換!("uei$", "ui"),
        變換!("uen$", "un"),

        // 聲母獨用時補足缺省韻母
        // ⟨bu, pu, fu⟩
        變換!("^([bpf])$", "${1}u"),
        // ⟨de, te, ne, le, ge, ke, he⟩
        // 特別地，⟨me⟩ 對應常用字「麼·么」
        變換!("^([mdtnlgkh])$", "${1}e"),

        // 檢查拼音音節，通過檢查則追加隔音符號
        變換!("^([bpm])([iu]|a|i?e|o|[ae]i|i?ao|[oi]u|i?an|[ie]n|[ei]ng|ang|ong)$", "$1$2'"),
        變換!("^([fw])(u|a|o|[ae]i|ao|ou|an|en|eng|ang|ong)$", "$1$2'"),
        變換!("^([dt])([iu]|i?a|i?e|uo|[aeu]i|i?ao|[oi]u|[iu]?an|[ue]n|[ei]ng|ang|ong)$", "$1$2'"),
        變換!("^([nl])([iuv]|i?a|[iv]?e|u?o|[aeu]i|i?ao|[oi]u|[iu]?an|[iue]n|[ei]ng|i?ang|ong)$", "$1$2'"),
        變換!("^([gkh])(u|u?a|e|uo|u?ai|[ue]i|ao|ou|u?an|[ue]n|eng|u?ang|ong)$", "$1$2'"),
        變換!("^([zcs]h?|r)([iu]|u?a|e|uo|u?ai|[ue]i|ao|ou|u?an|[ue]n|eng|u?ang|ong)$", "$1$2'"),
        變換!("^([jqxy])([iu]|i?a|[iu]?e|o|i?ao|[oi]u|[iu]?an|[iu]n|ing|i?ang|i?ong)$", "$1$2'"),
        變換!("^([aeo]|[ae]i|ao|ou|[ae]ng?|er)$", "$1'"),
        // 消除不構成合法音節的並擊組合
        消除!("^[A-Za-z]+$"),
        轉寫!("v", "ü"),
        // 顯示單個音節不需要加隔音符號
        變換!("^(.*)'$", "$1"),
    ]);

    static ref 拼音轉並擊: Box<[拼寫運算<'static>]> = Box::new([
        // 缺省韻母
        變換!("^bu$", "B"),
        變換!("^pu$", "P"),
        變換!("^me$", "FB"),
        變換!("^fu$", "F"),
        變換!("^de$", "D"),
        變換!("^te$", "T"),
        變換!("^ne$", "LD"),
        變換!("^le$", "L"),
        變換!("^ge$", "G"),
        變換!("^ke$", "K"),
        變換!("^he$", "H"),
        變換!("^zhi$", "ZF"),
        變換!("^chi$", "CL"),
        變換!("^shi$", "SH"),
        變換!("^ri$", "HG"),
        變換!("^zi$", "Z"),
        變換!("^ci$", "C"),
        變換!("^si$", "S"),
        變換!("^er$", "R"),
        // 韻母的並擊碼
        變換!("^a$", "AE"),
        變換!("a$", "A"),
        變換!("ao$", "AO"),
        變換!("o$", "O"),
        變換!("y?ue$", "ÜE"),
        變換!("e$", "E"),
        變換!("ai$", "AR"),
        變換!("^wei$|ui$", "UR"),
        變換!("^ei$", "RE"),
        變換!("^([zcsr]h?)ei$", "${1}RE"),
        變換!("ei$", "R"),
        變換!("^you$|iou$|iu$", "IR"),
        變換!("ou$", "RO"),
        變換!("an$", "AN"),
        變換!("^yin$|in$", "IN"),
        變換!("^yun$|ün$|vn$", "ÜN"),
        變換!("^([jqx])un$", "${1}ÜN"),
        變換!("^wen$|uen$|un$", "UN"),
        變換!("en$", "N"),
        變換!("wang$|uang$", "UARO"),
        變換!("ang$", "ANE"),
        變換!("^ying$|ing$", "INE"),
        變換!("^yong$|iong$", "IRO"),
        變換!("^weng$|ueng$|ong$", "URO"),
        變換!("eng$", "NE"),
        變換!("^([jqx])u", "${1}Ü"),
        變換!("^yu|ü|v", "Ü"),
        變換!("^yi?|ii?", "I"),
        變換!("^wu?|u", "U"),
        // 聲母的並擊碼
        變換!("^b", "B"),
        變換!("^p", "P"),
        變換!("^m", "FB"),
        變換!("^f", "F"),
        變換!("^d", "D"),
        變換!("^t", "T"),
        變換!("^n", "LD"),
        變換!("^l", "L"),
        變換!("^j-?$", "GI"),
        變換!("^q-?$", "KI"),
        變換!("^x-?$", "HI"),
        變換!("^[gj]", "G"),
        變換!("^[kq]", "K"),
        變換!("^[hx]", "H"),
        變換!("^zh", "ZF"),
        變換!("^ch", "CL"),
        變換!("^sh", "SH"),
        變換!("^z", "Z"),
        變換!("^c", "C"),
        變換!("^s", "S"),
        變換!("^r", "HG"),
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
        // 尖音，演示指法用。其中韻母 i 雙寫
        regex!("^([zcs])(ii|[iv]e?|üe?|i?ao|iu|[iv]a?n|üa?n|ia?ng|iong)$").deref(),
        regex!("^([aeo]|[ae]i|ao|ou|[ae]ng?|er)$").deref(),
        // 聲母
        regex!("^([bpmfdtnlgkhjqxr]|[zcs]h?)-?$").deref(),
        // 非音節形式的韻母
        regex!("^([yw])-?$").deref(),
        regex!("^-?([iuv]|[iu]?[ao]|[iuv]?e|üe?|u?[ae]i|ui|i?ao|i?ou|iu|[iuv]?an|üa?n|[iuv]n|u?en|[iu]?ang|ing|u?eng|i?ong)?$").deref(),
    ]);
}

const 宮保拼音盤面: 盤面定義<'static> = 盤面![
    [ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ],
    [ 空 {中:C, 上:_, 下:_, 左:_, 右:"ĉ"} {中:L, 上:_, 下:_, 左:"ĉ", 右:"n"} {中:D, 上:_, 下:_, 左:"n", 右:_} T 空 U {中:R, 上:"er", 下:_, 左:"-i", 右:"-u"} O 空 _ _ ],
    [ 空 {中:S, 上:_, 下:_, 左:_, 右:"ŝ"} {中:H, 上:"x", 下:_, 左:"ŝ", 右:"r"} {中:G, 上:_, 下:_, 左:"r", 右:"j"} {中:K, 上:_, 下:_, 左:_, 右:"q"} 空 I {中:N, 上:_, 下:_, 左:"-n", 右:"-ŋ"} {中:E, 上:_, 下:_, 左:"-ŋ", 右:_} _ _ _ ],
    [ 空 {中:Z, 上:_, 下:_, 左:_, 右:"ẑ"} {中:F, 上:_, 下:_, 左:"ẑ", 右:"m"} {中:B, 上:_, 下:_, 左:"m", 右:_} P 空 Ü _ _ _ ],
    [ A _ A ]
];

const 宮保拼音鍵盤佈局: 鍵盤佈局 = 鍵盤佈局 {
    盤面: &[基本盤面, 上檔盤面, 大寫字母盤面, 宮保拼音盤面],
    默認盤面: 盤面選擇碼(4),
    首選配列: 配列::正交直列,
};

pub fn 宮保拼音輸入方案(_環境: 輸入方案環境) -> 輸入方案定義<'static> {
    輸入方案定義 {
        名稱: "宮保拼音",
        佈局: &宮保拼音鍵盤佈局,
        指法: 觸鍵方式::並擊,
        編碼法: 碼表格式::並擊,
        字根表: 並擊鍵序,
        轉寫法: 轉寫法定義 {
            輸入碼表示: &[],
            輸入碼鍵位: &[],
            拼式轉寫規則: &並擊轉拼音,
            字根拆分規則: &拼音轉並擊,
            拼式驗證規則: &貌似拼音,
            邊界判定: 邊界判定規則 {
                分隔鍵: &[],
                起始鍵: &[],
                終止鍵: &[],
            },
        },
        動態切換: &[],
    }
}
