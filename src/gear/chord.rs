use keyberon::key_code::KeyCode;
use leptos::prelude::*;

use crate::action::動作;
use crate::definition::{輸入方案定義, 鍵組};
use crate::gear::assignment::對照輸入碼;

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

    pub fn 並擊完成(&mut self) {}
}

#[allow(clippy::type_complexity)]
pub fn 並擊機關(
    方案: Signal<輸入方案定義<'static>>,
    目標輸入碼: Signal<Option<對照輸入碼>>,
) -> (
    // 並擊狀態流
    ReadSignal<並擊狀態>,
    // 並擊狀態變更
    WriteSignal<並擊狀態>,
    // 實況並擊碼
    Signal<String>,
    // 並擊所得拼音
    Signal<Option<String>>,
    // 反查所得並擊碼
    Signal<Option<String>>,
    // 反查鍵位
    Signal<Option<鍵組>>,
    // 並擊開始
    Signal<bool>,
    // 並擊完成
    Signal<bool>,
    // 並擊成功
    Signal<bool>,
    // 重置並擊狀態
    impl 動作,
) {
    let (並擊狀態流, 並擊狀態變更) = signal(並擊狀態::new());

    let 重置並擊狀態 = move || 並擊狀態變更.write().重置();

    let 實況並擊碼 =
        Signal::derive(move || 方案.read().寫成字根碼(&並擊狀態流.read().累計擊鍵));
    let 並擊所得拼音 =
        Memo::new(move |_| 方案.read().轉寫法.字根碼轉寫爲拼式(&實況並擊碼()));

    let 反查所得並擊碼 = Memo::new(move |_| {
        目標輸入碼
            .read()
            .as_ref()
            .and_then(|對照碼| 對照碼.反查字根碼(&方案.read().轉寫法))
    });
    let 反查鍵位 = Signal::derive(move || {
        反查所得並擊碼
            .read()
            .as_deref()
            .map(|並擊碼| 方案.read().讀出鍵位(並擊碼))
    });

    let 並擊開始 = Signal::derive(move || !並擊狀態流.read().實時落鍵.0.is_empty());
    let 並擊完成 =
        Signal::derive(move || 並擊狀態流.read().實時落鍵.0.is_empty() && !實況並擊碼().is_empty());

    let 並擊成功 = Signal::derive(move || {
        // 拼音一致即爲成功，允許並擊碼不同
        目標輸入碼()
            .and_then(|輸入碼| 輸入碼.轉寫碼原文)
            .is_some_and(|查得| 並擊所得拼音().is_some_and(|擊得| 查得 == 擊得))
            // 拼音爲非音節形式的聲母、韻母，須比較並擊碼
            || 反查所得並擊碼().is_some_and(|查得| 查得 == 實況並擊碼())
    });

    (
        並擊狀態流,
        並擊狀態變更,
        實況並擊碼,
        並擊所得拼音.into(),
        反查所得並擊碼.into(),
        反查鍵位,
        並擊開始,
        並擊完成,
        並擊成功,
        重置並擊狀態,
    )
}
