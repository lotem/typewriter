use keyberon::key_code::KeyCode;

pub type 盤面選擇碼 = u64;

pub type 盤面刻印 = (盤面選擇碼, &'static str);

pub struct 鍵的定義 {
    pub 鍵碼: KeyCode,
    字符映射: &'static [盤面刻印],
}

impl 鍵的定義 {
    pub fn 選擇盤面(&self, 目標盤面: 盤面選擇碼) -> Option<盤面刻印> {
        self.字符映射
            .iter()
            .rfind(|盤面| (目標盤面 & 盤面.0) == 盤面.0)
            .copied()
    }
}

const Q: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Q,
    字符映射: &[(0, "q"), (1, "Q"), (2, "")],
};
const A: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::A,
    字符映射: &[(0, "a"), (1, "A"), (2, "")],
};
const Z: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Z,
    字符映射: &[(0, "z"), (1, "Z"), (2, "")],
};
const W: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::W,
    字符映射: &[(0, "w"), (1, "W"), (2, "C")],
};
const S: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::S,
    字符映射: &[(0, "s"), (1, "S"), (2, "S")],
};
const X: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::X,
    字符映射: &[(0, "x"), (1, "X"), (2, "Z")],
};
const E: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::E,
    字符映射: &[(0, "e"), (1, "E"), (2, "L")],
};
const D: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::D,
    字符映射: &[(0, "d"), (1, "D"), (2, "H")],
};
const C: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::C,
    字符映射: &[(0, "c"), (1, "C"), (2, "F")],
};
const R: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::R,
    字符映射: &[(0, "r"), (1, "R"), (2, "D")],
};
const F: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::F,
    字符映射: &[(0, "f"), (1, "F"), (2, "G")],
};
const V: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::V,
    字符映射: &[(0, "v"), (1, "V"), (2, "B")],
};
const T: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::T,
    字符映射: &[(0, "t"), (1, "T"), (2, "T")],
};
const G: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::G,
    字符映射: &[(0, "g"), (1, "G"), (2, "K")],
};
const B: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::B,
    字符映射: &[(0, "b"), (1, "B"), (2, "P")],
};
const Y: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Y,
    字符映射: &[(0, "y"), (1, "Y"), (2, "")],
};
const H: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::H,
    字符映射: &[(0, "h"), (1, "H"), (2, "")],
};
const N: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::N,
    字符映射: &[(0, "n"), (1, "N"), (2, "")],
};
const U: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::U,
    字符映射: &[(0, "u"), (1, "U"), (2, "U")],
};
const J: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::J,
    字符映射: &[(0, "j"), (1, "J"), (2, "I")],
};
const M: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::M,
    字符映射: &[(0, "m"), (1, "M"), (2, "Ü")],
};
const I: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::I,
    字符映射: &[(0, "i"), (1, "I"), (2, "R")],
};
const K: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::K,
    字符映射: &[(0, "k"), (1, "K"), (2, "N")],
};
const Comma: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Comma,
    字符映射: &[(0, ","), (1, "<")],
};
const O: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::O,
    字符映射: &[(0, "o"), (1, "O"), (2, "O")],
};
const L: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::L,
    字符映射: &[(0, "l"), (1, "L"), (2, "E")],
};
const Dot: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Dot,
    字符映射: &[(0, "."), (1, ">")],
};
const P: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::P,
    字符映射: &[(0, "p"), (1, "P"), (2, "")],
};
const SColon: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::SColon,
    字符映射: &[(0, ";"), (1, ":")],
};
const Slash: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Slash,
    字符映射: &[(0, "/"), (1, "?")],
};
const Space: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Space,
    字符映射: &[(0, " "), (2, "A")],
};

pub const 鍵盤矩陣: &[&[鍵的定義]] = &[
    &[Q, W, E, R, T, Y, U, I, O, P],
    &[A, S, D, F, G, H, J, K, L, SColon],
    &[Z, X, C, V, B, N, M, Comma, Dot, Slash],
    &[Space],
];
