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
    轉寫(HashMap<char, char>),
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
        拼寫運算::轉寫(zip($左字表.chars(), $右字表.chars()).collect())
    };
}

lazy_static! {
    static ref 並擊轉拼音: Vec<拼寫運算::<'static>> = vec![
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
        變換!("ÜN$", "ün"),
        變換!("UN$", "uen"),
        變換!("IN$", "in"),
        變換!("N$", "en"),

        變換!("IAR$", "iao"),
        變換!("AR$", "ai"),
        變換!("RE$", "ei"),
        變換!("UR$", "uei"),
        變換!("RO$", "ou"),
        變換!("IR$", "iou"),
        變換!("AO$", "ao"),
        變換!("AE$", "a"),

        變換!("^([dtnlgkhzcsr]h?)O$", "${1}ou"),
        變換!("^([bpmfdtnlgkh])E$", "${1}ei"),

        轉寫!("AOEIUÜ", "aoeiuü"),

        // 漢語拼音方案的拼寫規則
        變換!("^i(ng?)$", "yi$1"),
        變換!("^i$", "yi"),
        變換!("^i", "y"),
        變換!("^ong$", "weng"),
        變換!("^u$", "wu"),
        變換!("^u", "w"),
        變換!("^ü", "yu"),
        變換!("^([jqx])ü", "${1}u"),
        // 一些容錯
        變換!("^([bpmf])uo$", "${1}o"),
        變換!("^([nl])uei$", "${1}ei"),
        變換!("^([nl])iong$", "${1}ong"),
        變換!("io$", "iao"),
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
    ];
}

pub type 鍵組 = BTreeSet<KeyCode>;

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
        if self.累計擊鍵.is_empty() {
            return String::new();
        }
        並擊鍵序
            .iter()
            .filter(|鍵| self.累計擊鍵.contains(&鍵.鍵碼))
            .map(|鍵| 鍵.輸入碼)
            .collect::<String>()
    }

    pub fn 並擊變換(並擊序列: &str) -> Option<String> {
        if 並擊序列.is_empty() {
            return None;
        }
        let mut 運算結果 = 並擊序列.to_owned();
        for 運算 in &*並擊轉拼音 {
            運算結果 = match 運算 {
                拼寫運算::變換 {
                    ref 模式, 替換文字
                } => 模式.replace_all(&運算結果, *替換文字).to_string(),
                拼寫運算::轉寫(ref 字符映射) => 運算結果
                    .chars()
                    .map(|字符| 字符映射.get(&字符).copied().unwrap_or(字符))
                    .collect::<String>(),
            };
        }
        (!運算結果.is_empty()).then_some(運算結果)
    }
}
