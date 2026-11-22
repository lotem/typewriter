use leptos::prelude::*;

use crate::action::動作;
use crate::definition::鍵組;
use crate::gear::{
    assignment::作業機關輸出信號, layout::盤面選擇碼, theory::輸入方案機關輸出信號
};
use crate::key_code::KeyCode;

pub struct 並擊狀態 {
    pub 實時落鍵: 鍵組,
    pub 累計擊鍵: 鍵組,
    pub 並擊計次: usize,
}

impl 並擊狀態 {
    pub fn new() -> Self {
        並擊狀態 {
            實時落鍵: 鍵組::new(),
            累計擊鍵: 鍵組::new(),
            並擊計次: 0,
        }
    }

    pub fn 落鍵(&mut self, 鍵碼: KeyCode) {
        if self.實時落鍵.0.is_empty() {
            self.並擊開始();
        }
        self.實時落鍵.0.insert(鍵碼);
        self.累計擊鍵.0.insert(鍵碼);
    }

    pub fn 抬鍵(&mut self, 鍵碼: KeyCode) {
        self.實時落鍵.0.remove(&鍵碼);
        if self.實時落鍵.0.is_empty() {
            self.並擊完成();
        }
    }

    pub fn 重置(&mut self) {
        self.實時落鍵.0.clear();
        self.累計擊鍵.0.clear();
    }

    pub fn 並擊開始(&mut self) {
        self.重置();
    }

    pub fn 並擊完成(&mut self) {
        self.並擊計次 = self.並擊計次.wrapping_add(1);
    }
}

/// [並擊] 失誤結算時的編碼回顯對照描述: 並列展示實際擊出的錯碼與應擊目標碼.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct 並擊對照表示 {
    /// 剛纔實際擊出的並擊字根碼 (若僅誤觸無定義鍵則爲 `"∅"`)
    pub 實擊並擊碼: String,
    /// 目標應擊的並擊字根碼
    pub 目標並擊碼: String,
    /// 剛纔實際擊出的合法轉寫拼音 (已帶 `⟨...⟩`; 不構成合法音節時爲 `None`)
    pub 實擊轉寫碼: Option<String>,
    /// 目標應擊的轉寫拼音 (已帶 `⟨...⟩`)
    pub 目標轉寫碼: Option<String>,
}

pub type 並擊重置動作 = impl 動作;

#[derive(Clone)]
pub struct 並擊機關輸出信號 {
    pub 並擊狀態流: ReadSignal<並擊狀態>,
    pub 並擊狀態變更: WriteSignal<並擊狀態>,
    pub 重置並擊狀態: 並擊重置動作,
    pub 實況並擊碼: Memo<String>,
    pub 並擊所得拼音: Signal<Option<String>>,
    pub 反查所得並擊碼: Memo<Option<String>>,
    pub 反查鍵位: Signal<Option<鍵組>>,
    pub 並擊完成: Signal<bool>,
    pub 並擊成功: Memo<bool>,
    pub 並擊失誤: Signal<bool>,
    pub 並擊計次: Signal<usize>,
    pub 並擊對照: Signal<Option<並擊對照表示>>,
}

#[define_opaque(並擊重置動作)]
pub fn 並擊機關(
    方案: &輸入方案機關輸出信號,
    作業: &作業機關輸出信號,
) -> 並擊機關輸出信號 {
    let 方案定義 = 方案.方案定義;
    let 目標輸入碼片段 = 作業.目標輸入碼片段;
    let 作業進度完成 = 作業.作業進度完成;

    let (並擊狀態流, 並擊狀態變更) = signal(並擊狀態::new());

    let 重置並擊狀態 = move || {
        並擊狀態變更.write().重置();
    };

    let 實況並擊碼 =
        Memo::new(move |_| 方案定義.read().寫成字根碼(&並擊狀態流.read().累計擊鍵));
    let 並擊所得拼音 =
        Signal::derive(move || 方案定義.read().轉寫法.字根碼轉寫爲拼式(&實況並擊碼()));

    let 反查所得並擊碼 = Memo::new(move |_| {
        目標輸入碼片段
            .read()
            .as_ref()
            .and_then(|對照碼| 對照碼.反查字根碼(&方案定義.read().轉寫法))
    });
    let 反查鍵位 = Signal::derive(move || {
        反查所得並擊碼
            .read()
            .as_deref()
            .map(|並擊碼| 方案定義.read().讀出鍵位(並擊碼, &盤面選擇碼(0)))
    });

    let _並擊開始 = Signal::derive(move || !並擊狀態流.read().實時落鍵.0.is_empty());

    let 並擊完成 = Signal::derive(move || {
        let 狀態 = 並擊狀態流.read();
        狀態.實時落鍵.0.is_empty() && !狀態.累計擊鍵.0.is_empty()
    });

    let 並擊成功 = Memo::new(move |_| {
        // 拼音一致即爲成功, 允許並擊碼不同
        目標輸入碼片段()
            .is_some_and(|輸入碼| {
                輸入碼.字根碼原文.is_some_and(|查得| 查得 == 實況並擊碼()) ||
                輸入碼.轉寫碼原文.is_some_and(|查得| 並擊所得拼音().is_some_and(|擊得| 查得 == 擊得))
            })
            // 拼音爲非音節形式的聲母, 韻母, 須比較並擊碼
            || 反查所得並擊碼().is_some_and(|查得| 查得 == 實況並擊碼())
    });

    let 並擊失誤 = Signal::derive(move || {
        !作業進度完成() && 目標輸入碼片段.read().is_some() && 並擊完成() && !並擊成功()
    });

    let 並擊計次 = Signal::derive(move || 並擊狀態流.read().並擊計次);

    let 並擊對照 = Signal::derive(move || {
        if !並擊失誤() {
            return None;
        }
        let 實擊 = 實況並擊碼();
        let 實擊並擊碼 = if 實擊.is_empty() {
            String::from("∅")
        } else {
            實擊
        };
        let 目標並擊碼 = 反查所得並擊碼().unwrap_or_default();
        let 目標原文 = 目標輸入碼片段().and_then(|輸入碼| 輸入碼.轉寫碼原文);
        let 實擊原文 = 並擊所得拼音().filter(|實拼| 目標原文.as_deref() != Some(實拼.as_str()));
        Some(並擊對照表示 {
            實擊並擊碼,
            目標並擊碼,
            實擊轉寫碼: 實擊原文.map(|轉寫| format!("⟨{轉寫}⟩")),
            目標轉寫碼: 目標原文.map(|轉寫| format!("⟨{轉寫}⟩")),
        })
    });

    並擊機關輸出信號 {
        並擊狀態流,
        並擊狀態變更,
        重置並擊狀態,
        實況並擊碼,
        並擊所得拼音,
        反查所得並擊碼,
        反查鍵位,
        並擊完成,
        並擊成功,
        並擊失誤,
        並擊計次,
        並擊對照,
    }
}
