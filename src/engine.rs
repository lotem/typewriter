use keyberon::key_code::KeyCode;
use lazy_regex::Regex;
use leptos::logging::log;
use leptos::*;

use crate::gear::{
    assignment::{作業, 作業推進參數, 作業機關},
    caption::{字幕機關, 字幕段落},
    chord::{並擊機關, 並擊狀態, 鍵組},
    input::{焦點事件處理機關, 輸入事件處理機關},
    key_press::連擊機關,
    mode::{工作模式, 工作模式機關},
    theory::{方案選項, 輸入方案機關},
};
use crate::layout::盤面選擇碼;
use crate::spelling_algebra::{拼寫運算, 施展拼寫運算};

pub struct 鍵的定義<'a> {
    pub 輸入碼: &'a str,
    pub 鍵碼: KeyCode,
}

#[derive(Clone, Copy)]
pub struct 輸入方案定義<'a> {
    pub 名稱: &'a str,
    pub 盤面: 盤面選擇碼,
    pub 指法: 觸鍵方式,
    pub 字根表: &'a [鍵的定義<'a>],
    pub 轉寫法: 轉寫法定義<'a>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum 觸鍵方式 {
    連擊,
    並擊,
}

#[derive(Clone, Copy)]
pub struct 轉寫法定義<'a> {
    pub 拼式轉寫規則: &'a [拼寫運算<'a>],
    pub 字根拆分規則: &'a [拼寫運算<'a>],
    pub 拼式驗證規則: &'a [&'a Regex],
}

pub trait 判定鍵位 {
    fn 有無鍵位(&self) -> bool;
    fn 包含鍵位(&self, 鍵碼: &KeyCode) -> bool;
}

impl 判定鍵位 for &鍵組 {
    fn 有無鍵位(&self) -> bool {
        !self.0.is_empty()
    }

    fn 包含鍵位(&self, 鍵碼: &KeyCode) -> bool {
        self.0.contains(鍵碼)
    }
}

impl 判定鍵位 for KeyCode {
    fn 有無鍵位(&self) -> bool {
        *self != KeyCode::No
    }

    fn 包含鍵位(&self, 鍵碼: &KeyCode) -> bool {
        self == 鍵碼
    }
}

impl 輸入方案定義<'_> {
    pub fn 尋得字根(&self, 字根: &str) -> Option<&鍵的定義> {
        self.字根表.iter().find(|鍵| 鍵.輸入碼 == 字根)
    }

    pub fn 讀出鍵位(&self, 字根碼: &str) -> 鍵組 {
        鍵組(
            self.字根表
                .iter()
                .filter(|鍵| 字根碼.contains(鍵.輸入碼))
                .map(|鍵| 鍵.鍵碼)
                .collect(),
        )
    }

    pub fn 寫成字根碼(&self, 鍵位: impl 判定鍵位) -> String {
        if !鍵位.有無鍵位() {
            String::new()
        } else {
            self.字根表
                .iter()
                .filter(|鍵| 鍵位.包含鍵位(&鍵.鍵碼))
                .map(|鍵| 鍵.輸入碼)
                .collect::<String>()
        }
    }
}

impl 轉寫法定義<'_> {
    pub fn 字根碼轉寫爲拼式(&self, 字根碼: &str) -> Option<String> {
        施展拼寫運算(字根碼, self.拼式轉寫規則)
    }

    pub fn 拼式拆分爲字根碼(&self, 轉寫碼: &str) -> Option<String> {
        施展拼寫運算(轉寫碼, self.字根拆分規則)
    }

    pub fn 驗證拼式(&self, 待驗證拼式: &str) -> bool {
        self.拼式驗證規則.iter().any(|r| r.is_match(待驗證拼式))
    }
}

pub fn 微觀引擎() -> (
    // 反查鍵位
    Signal<Option<鍵組>>,
    // 有無輸入碼
    Signal<bool>,
    // 指法
    Signal<觸鍵方式>,
    // 並擊狀態流
    ReadSignal<並擊狀態>,
    // 現行工作模式
    ReadSignal<工作模式>,
    // 按進度顯示字幕段落
    Signal<Option<(String, String, String)>>,
    // 輸入正確
    Signal<bool>,
    // 當前作業
    ReadSignal<作業>,
    // 佈置作業
    WriteSignal<作業>,
    // 回顯輸入碼
    Signal<String>,
    // 回顯轉寫碼
    Signal<Option<String>>,
    // 現行方案
    ReadSignal<方案選項>,
    // 選用方案
    WriteSignal<方案選項>,
    // 方案定義
    Signal<輸入方案定義<'static>>,
    // 開啓練習題選單
    impl Fn() + Copy + 'static,
    // 開啓反查輸入
    impl Fn() + Copy + 'static,
    // 關閉輸入欄
    impl Fn() + Copy + 'static,
) {
    let (現行方案, 選用方案, 方案定義) = 輸入方案機關();

    let 指法 = Signal::derive(move || 方案定義.with(|方案| 方案.指法));

    let (
        當前作業,
        佈置作業,
        有無作業,
        作業進度,
        作業進度完成,
        反查輸入碼序列,
        目標輸入碼,
        重置作業進度,
        作業推進,
        作業回退,
    ) = 作業機關(現行方案, 方案定義);

    let (分段字幕, 當前段落, 按進度顯示字幕段落) =
        字幕機關(當前作業, 作業進度, 反查輸入碼序列, 指法);

    let (
        _連擊狀態流,
        連擊狀態變更,
        實況字根碼,
        _反查所得字根碼,
        連擊比對成功,
        連擊輸入碼,
        _更新連擊輸入碼,
        清空連擊輸入碼,
        回退連擊輸入碼,
    ) = 連擊機關(方案定義, 目標輸入碼);

    let (
        並擊狀態流,
        並擊狀態變更,
        實況並擊碼,
        並擊所得拼音,
        反查所得並擊碼,
        反查鍵位,
        _並擊開始,
        並擊完成,
        並擊成功,
        重置並擊狀態,
    ) = 並擊機關(方案定義, 目標輸入碼);

    let 重置輸入狀態 = move || match 指法() {
        觸鍵方式::連擊 => 清空連擊輸入碼(),
        觸鍵方式::並擊 => 重置並擊狀態(),
    };

    let (現行工作模式, 開啓反查輸入, 開啓練習題選單, 開啓方案選單, 關閉輸入欄) =
        工作模式機關(現行方案, 作業進度完成, 佈置作業, 重置作業進度, 重置輸入狀態);

    let _ = watch(
        現行工作模式,
        move |新, 舊, _| {
            log!("工作模式: {:?} -> {:?}", 舊, 新);
        },
        false,
    );

    焦點事件處理機關(重置並擊狀態);

    let 處理功能鍵 = move |鍵碼: KeyCode| match 鍵碼 {
        KeyCode::Escape => {
            match 現行工作模式() {
                工作模式::錄入 => {
                    if 作業進度() != 0 {
                        重置作業進度();
                        重置輸入狀態();
                    } else {
                        開啓練習題選單();
                    }
                }
                _ => 關閉輸入欄(),
            }
            true
        }
        KeyCode::Tab => {
            if 現行工作模式() == 工作模式::錄入 {
                if 作業推進(作業推進參數 {
                    段落: 當前段落().map(|字幕段落(起, 止, _)| (起, 止)),
                    迴轉: true,
                })
                .is_ok()
                {
                    重置輸入狀態();
                }
            } else {
                關閉輸入欄();
            }
            true
        }
        KeyCode::BSpace => {
            if 現行工作模式() == 工作模式::錄入 {
                match 指法() {
                    觸鍵方式::連擊 => {
                        if 有無作業() {
                            let _不看結果 = 作業回退();
                        }
                        回退連擊輸入碼();
                    }
                    觸鍵方式::並擊 => {
                        if 並擊完成() || 作業回退().is_ok() {
                            重置並擊狀態();
                        }
                    }
                }
                return true;
            }
            false
        }
        KeyCode::Enter => {
            if 現行工作模式() == 工作模式::錄入 {
                開啓反查輸入();
            } else {
                關閉輸入欄();
            }
            true
        }
        KeyCode::Grave => {
            match 現行工作模式() {
                工作模式::選擇輸入方案 => 關閉輸入欄(),
                _ => 開啓方案選單(),
            }
            true
        }
        _ => false,
    };

    let 擊中目標 = move || match 指法() {
        觸鍵方式::連擊 => 連擊比對成功(),
        觸鍵方式::並擊 => 並擊完成() && 並擊成功(),
    };
    let 批閱作業 = move || {
        // 擊中目標輸入碼後反查下一個輸入碼
        let 分段落則迴轉 = 分段字幕.with(|衆段落| 衆段落.len() > 1);
        擊中目標() && 作業推進(作業推進參數::步進(分段落則迴轉)).is_ok()
    };
    let 另起一段 = move || {
        with!(|作業進度, 當前段落| {
            當前段落
                .as_ref()
                .is_some_and(|字幕段落(段落起始, _, _)| 作業進度 == 段落起始)
        })
    };
    let 既然落鍵 = move |鍵碼| {
        // 繼續擊鍵時消除已完成的反查作業
        if 作業進度完成() {
            佈置作業(作業::自習(現行方案()));
        }
        if 現行工作模式() == 工作模式::錄入 {
            並擊狀態變更.update(|並擊| 並擊.落鍵(鍵碼));
            if 指法() == 觸鍵方式::連擊 {
                連擊狀態變更.update(|連擊| *連擊 = 連擊.擊發(鍵碼));
                if 批閱作業() && 另起一段() {
                    清空連擊輸入碼();
                }
            }
        }
    };
    let 既然抬鍵 = move |鍵碼| {
        if 現行工作模式() == 工作模式::錄入 {
            並擊狀態變更.update(|並擊| 並擊.抬鍵(鍵碼));
        }
        match 指法() {
            觸鍵方式::連擊 => {
                // 顯示並擊動態, 抬鍵後清除並擊結果
                if 並擊完成() && !作業進度完成() {
                    重置並擊狀態();
                }
            }
            觸鍵方式::並擊 => {
                // 推進到下一題時, 清除上一題的並擊結果
                // 但在最後一題完成後停下顯示結果
                if 批閱作業() && !作業進度完成() {
                    重置並擊狀態();
                }
            }
        }
    };

    輸入事件處理機關(處理功能鍵, 既然落鍵, 既然抬鍵);

    let 有無輸入碼 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => !實況字根碼().is_empty(),
        觸鍵方式::並擊 => !實況並擊碼().is_empty(),
    });
    let 完成一詞 = move || {
        按進度顯示字幕段落.with(|段落| {
            段落
                .as_ref()
                .is_some_and(|(_, 下一字, _)| ["", " "].contains(&下一字.as_str()))
        })
    };
    let 輸入正確 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => 完成一詞(),
        觸鍵方式::並擊 => 並擊完成() && 並擊成功(),
    });
    let 回顯輸入碼 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => with!(|連擊輸入碼| {
            let 輸入碼 = 連擊輸入碼.join("");
            match 輸入碼.as_str() {
                "" | "␣" => 輸入碼,
                _ => 輸入碼 + "‸",
            }
        }),
        觸鍵方式::並擊 => 反查所得並擊碼().unwrap_or_else(實況並擊碼),
    });
    let 回顯轉寫碼 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => None,
        觸鍵方式::並擊 => {
            目標輸入碼()
                .and_then(|輸入碼| 輸入碼.轉寫碼原文)
                .or_else(|| 並擊所得拼音().to_owned())
                // 加尖括弧表示拉丁文轉寫
                .map(|轉寫| format!("⟨{轉寫}⟩"))
        }
    });

    (
        反查鍵位,
        有無輸入碼,
        指法,
        並擊狀態流,
        現行工作模式,
        按進度顯示字幕段落,
        輸入正確,
        當前作業,
        佈置作業,
        回顯輸入碼,
        回顯轉寫碼,
        現行方案,
        選用方案,
        方案定義,
        開啓練習題選單,
        開啓反查輸入,
        關閉輸入欄,
    )
}
