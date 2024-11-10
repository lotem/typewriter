use keyberon::key_code::KeyCode;
use lazy_regex::regex;
use lazy_static::lazy_static;
use maybe_owned::MaybeOwned;
use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use std::iter::zip;

pub struct 鍵的定義<'a> {
    輸入碼: &'a str,
    鍵碼: KeyCode,
}

const S: 鍵的定義 = 鍵的定義 {
    輸入碼: "S",
    鍵碼: KeyCode::S,
};
const C: 鍵的定義 = 鍵的定義 {
    輸入碼: "C",
    鍵碼: KeyCode::W,
};
const Z: 鍵的定義 = 鍵的定義 {
    輸入碼: "Z",
    鍵碼: KeyCode::X,
};
const H: 鍵的定義 = 鍵的定義 {
    輸入碼: "H",
    鍵碼: KeyCode::D,
};
const L: 鍵的定義 = 鍵的定義 {
    輸入碼: "L",
    鍵碼: KeyCode::E,
};
const F: 鍵的定義 = 鍵的定義 {
    輸入碼: "F",
    鍵碼: KeyCode::C,
};
const G: 鍵的定義 = 鍵的定義 {
    輸入碼: "G",
    鍵碼: KeyCode::F,
};
const D: 鍵的定義 = 鍵的定義 {
    輸入碼: "D",
    鍵碼: KeyCode::R,
};
const B: 鍵的定義 = 鍵的定義 {
    輸入碼: "B",
    鍵碼: KeyCode::V,
};
const K: 鍵的定義 = 鍵的定義 {
    輸入碼: "K",
    鍵碼: KeyCode::G,
};
const T: 鍵的定義 = 鍵的定義 {
    輸入碼: "T",
    鍵碼: KeyCode::T,
};
const P: 鍵的定義 = 鍵的定義 {
    輸入碼: "P",
    鍵碼: KeyCode::B,
};
const I: 鍵的定義 = 鍵的定義 {
    輸入碼: "I",
    鍵碼: KeyCode::J,
};
const U: 鍵的定義 = 鍵的定義 {
    輸入碼: "U",
    鍵碼: KeyCode::U,
};
const Ü: 鍵的定義 = 鍵的定義 {
    輸入碼: "Ü",
    鍵碼: KeyCode::M,
};
const A: 鍵的定義 = 鍵的定義 {
    輸入碼: "A",
    鍵碼: KeyCode::Space,
};
const N: 鍵的定義 = 鍵的定義 {
    輸入碼: "N",
    鍵碼: KeyCode::K,
};
const R: 鍵的定義 = 鍵的定義 {
    輸入碼: "R",
    鍵碼: KeyCode::I,
};
const E: 鍵的定義 = 鍵的定義 {
    輸入碼: "E",
    鍵碼: KeyCode::L,
};
const O: 鍵的定義 = 鍵的定義 {
    輸入碼: "O",
    鍵碼: KeyCode::O,
};
const 並擊鍵序: &[鍵的定義] = &[S, C, Z, H, L, F, G, D, B, K, T, P, I, U, Ü, A, N, R, E, O];
enum 拼寫運算<'a> {
    變換 {
        模式: MaybeOwned<'a, Regex>,
        替換文字: &'a str,
    },
    轉寫 {
        字符映射: HashMap<char, char>,
    },
    消除 {
        模式: MaybeOwned<'a, Regex>,
    },
}
macro_rules! 變換 {
    ($模式:literal, $替換文字:literal) => {
        拼寫運算::變換 {
            模式: regex!($模式).deref().into(),
            替換文字: $替換文字,
        }
    };
}

macro_rules! 轉寫 {
    ($左字表:literal, $右字表:literal) => {
        拼寫運算::轉寫 {
            字符映射: zip($左字表.chars(), $右字表.chars()).collect(),
        }
    };
}

macro_rules! 消除 {
    ($模式:literal) => {
        拼寫運算::消除 {
            模式: regex!($模式).deref().into(),
        }
    };
}

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

    static ref 拼音轉並擊: Box<[拼寫運算::<'static>]> = Box::new([
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
        變換!("ue$", "ÜE"),
        變換!("e$", "E"),
        變換!("ai$", "AR"),
        變換!("^wei$|ui$", "UR"),
        變換!("^ei$", "RE"),
        變換!("^([zcsr]h?)ei$", "${1}RE"),
        變換!("ei$", "R"),
        變換!("^you$|iou$|iu$", "IR"),
        變換!("ou$", "RO"),
        變換!("an$", "AN"),
        變換!("^yin|in$", "IN"),
        變換!("^yun|ün$|vn$", "ÜN"),
        變換!("^([jqx])un$", "${1}ÜN"),
        變換!("^wen$|uen$|un$", "UN"),
        變換!("en$", "N"),
        變換!("wang$|uang$", "UARO"),
        變換!("ang$", "ANE"),
        變換!("^ying$|ing$", "INE"),
        變換!("^yong$|iong$", "IRO"),
        變換!("^weng$|ueng$|ong$", "URO"),
        變換!("eng$", "NE"),
        變換!("^yu|ü|v", "Ü"),
        變換!("^yi?|i", "I"),
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
}

pub type 鍵組 = BTreeSet<KeyCode>;

pub fn 寫成並擊序列(並擊: &鍵組) -> String {
    if 並擊.is_empty() {
        return String::new();
    }
    並擊鍵序
        .iter()
        .filter(|鍵| 並擊.contains(&鍵.鍵碼))
        .map(|鍵| 鍵.輸入碼)
        .collect::<String>()
}

pub struct 並擊狀態 {
    pub 實時落鍵: 鍵組,
    pub 累計擊鍵: 鍵組,
}

impl 並擊狀態 {
    pub fn new() -> Self {
        並擊狀態 {
            實時落鍵: 鍵組::new(),
            累計擊鍵: 鍵組::new(),
        }
    }

    pub fn 落鍵(&mut self, 鍵碼: KeyCode) {
        if self.實時落鍵.is_empty() {
            self.並擊開始();
        }
        self.實時落鍵.insert(鍵碼);
        self.累計擊鍵.insert(鍵碼);
    }

    pub fn 抬鍵(&mut self, 鍵碼: KeyCode) {
        self.實時落鍵.remove(&鍵碼);
        if self.實時落鍵.is_empty() {
            self.並擊完成();
        }
    }

    pub fn 重置(&mut self) {
        self.實時落鍵.clear();
        self.累計擊鍵.clear();
    }

    pub fn 並擊開始(&mut self) {
        self.重置();
    }

    pub fn 並擊完成(&mut self) {}

    pub fn 並擊序列(&self) -> String {
        寫成並擊序列(&self.累計擊鍵)
    }

    pub fn 並擊變換(並擊碼: &str) -> Option<String> {
        拼寫運算(並擊碼, &並擊轉拼音)
    }
}

pub fn 反查變換(反查碼: &str) -> Option<鍵組> {
    let 反查結果 = 拼寫運算(反查碼, &拼音轉並擊)?;
    let 得一鍵組 = 並擊鍵序
        .iter()
        .filter(|鍵| 反查結果.contains(鍵.輸入碼))
        .map(|鍵| 鍵.鍵碼)
        .collect::<鍵組>();
    Some(得一鍵組)
}

fn 拼寫運算(原形: &str, 運算規則: &[拼寫運算]) -> Option<String> {
    if 原形.is_empty() {
        return None;
    }
    let mut 運算結果 = 原形.to_owned();
    for 運算 in 運算規則 {
        match 運算 {
            拼寫運算::變換 {
                ref 模式, 替換文字
            } => {
                運算結果 = 模式.replace_all(&運算結果, *替換文字).to_string();
            }
            拼寫運算::轉寫 { ref 字符映射 } => {
                運算結果 = 運算結果
                    .chars()
                    .map(|字符| 字符映射.get(&字符).copied().unwrap_or(字符))
                    .collect::<String>();
            }
            拼寫運算::消除 { ref 模式 } => {
                if 模式.is_match(&運算結果) {
                    return None;
                }
            }
        };
    }
    (!運算結果.is_empty()).then_some(運算結果)
}

fn 貌似拼音(s: &str) -> bool {
    [
        regex!("^([bpm])([iu]|a|i?e|o|[ae]i|i?ao|[oi]u|i?an|[ie]n|[ei]ng|ang|ong)$"),
        regex!("^([fw])(u|a|o|[ae]i|ao|ou|an|en|eng|ang|ong)$"),
        regex!("^([dt])([iu]|i?a|i?e|uo|[aeu]i|i?ao|[oi]u|[iu]?an|[ue]n|[ei]ng|ang|ong)$"),
        regex!(
            "^([nl])([iuv]|i?a|[iv]?e|üe?|u?o|[aeu]i|i?ao|[oi]u|[iu]?an|[iue]n|[ei]ng|i?ang|ong)$"
        ),
        regex!("^([gkh])(u|u?a|e|uo|u?ai|[ue]i|ao|ou|u?an|[ue]n|eng|u?ang|ong)$"),
        regex!("^([zcs]h?|r)([iu]|u?a|e|uo|u?ai|[ue]i|ao|ou|u?an|[ue]n|eng|u?ang|ong)$"),
        regex!("^([jqxy])([iu]|i?a|[iu]?e|o|i?ao|[oi]u|[iu]?an|[iu]n|ing|i?ang|i?ong)$"),
        regex!("^([aeo]|[ae]i|ao|ou|[ae]ng?|er)$"),
        // 聲母
        regex!("^([bpmfdtnlgkhjqxr]|[zcs]h?)-?$"),
        // 非音節形式的韻母
        regex!("^([yw])-?$"),
        regex!("^-?([iuv]|[iu]?[ao]|[iuv]?e|üe?|u?[ae]i|ui|i?ao|i?ou|iu|[iuv]?an|üa?n|[iuv]n|u?en|[iu]?ang|ing|u?eng|i?ong)?$"),
    ]
    .iter()
    .any(|r| r.is_match(s))
}

pub fn 解析拼音(長拼音: &str) -> Vec<String> {
    長拼音
        .split(&[' ', '\''][..])
        .filter(|&s| !s.is_empty() && 貌似拼音(s))
        .map(str::to_string)
        .collect()
}
