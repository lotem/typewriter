use keyberon::key_code::KeyCode;
use leptos::*;
use leptos::{
    ev::{keydown, keyup, KeyboardEvent},
    logging::log,
};
use leptos_use::{use_document, use_event_listener, use_window_focus};
use std::cmp::min;

use crate::drills::預設練習題;
use crate::engine::{並擊狀態, 解析輸入碼序列, 輸入碼, 鍵組};
use crate::key_code::網頁鍵值轉換;
use crate::layout::{盤面刻印, 盤面選擇碼, 鍵的定義, 鍵盤矩陣};
use crate::style::樣式;

const 宮保拼音盤面: 盤面選擇碼 = 盤面選擇碼(2);

const 退出鍵: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Escape,
    字符映射: &[(0, "退出")],
};
const 製表鍵: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Tab,
    字符映射: &[(0, "製表")],
};
const 退格鍵: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::BSpace,
    字符映射: &[(0, "退格")],
};
const 回車鍵: 鍵的定義 = 鍵的定義 {
    鍵碼: KeyCode::Enter,
    字符映射: &[(0, "回車")],
};

pub trait 鍵面標註法 {
    fn 刻印(&self) -> Option<String>;
    fn 是否空鍵(&self) -> bool;
    fn 是否後備盤面(&self) -> bool;
    fn 是否功能鍵(&self) -> bool;
    fn 是否空格(&self) -> bool;
}

pub trait 鍵面動態着色法 {
    fn 鍵位提示(&self, 鍵: &鍵的定義) -> bool;
    fn 是否落鍵(&self, 鍵: &鍵的定義) -> bool;
    fn 是否擊中(&self, 鍵: &鍵的定義) -> bool;
}

#[derive(Clone, Copy)]
struct 鍵面標註依據 {
    鍵: &'static 鍵的定義,
    目標盤面: 盤面選擇碼,
}

impl 鍵面標註依據 {
    fn 有效盤面(&self) -> Option<盤面刻印> {
        self.鍵.選擇盤面(self.目標盤面)
    }
}

impl 鍵面標註法 for 鍵面標註依據 {
    fn 刻印(&self) -> Option<String> {
        self.有效盤面().map(|盤面刻印| 盤面刻印.1.to_owned())
    }
    fn 是否空鍵(&self) -> bool {
        self.有效盤面().is_some_and(|盤面| 盤面.1.is_empty())
    }
    fn 是否後備盤面(&self) -> bool {
        self.有效盤面()
            .is_some_and(|盤面| 盤面.0 != self.目標盤面.0)
    }
    fn 是否功能鍵(&self) -> bool {
        [
            KeyCode::Escape,
            KeyCode::Tab,
            KeyCode::BSpace,
            KeyCode::Enter,
        ]
        .contains(&self.鍵.鍵碼)
    }
    fn 是否空格(&self) -> bool {
        self.鍵.鍵碼 == KeyCode::Space
    }
}

#[derive(Clone, Copy)]
struct 擊鍵動態 {
    目標並擊: Signal<Option<鍵組>>,
    實況並擊: Signal<並擊狀態>,
}

impl 鍵面動態着色法 for 擊鍵動態 {
    fn 鍵位提示(&self, 鍵: &鍵的定義) -> bool {
        self.目標並擊
            .with(|有冇| 有冇.as_ref().is_some_and(|並擊| 並擊.0.contains(&鍵.鍵碼)))
    }

    fn 是否落鍵(&self, 鍵: &鍵的定義) -> bool {
        self.實況並擊
            .with(|並擊| 並擊.實時落鍵.0.contains(&鍵.鍵碼))
    }

    fn 是否擊中(&self, 鍵: &鍵的定義) -> bool {
        self.實況並擊
            .with(|並擊| 並擊.累計擊鍵.0.contains(&鍵.鍵碼))
    }
}

#[derive(Clone, Copy)]
struct 功能鍵開關狀態 {
    實時工作模式: Signal<工作模式>,
}

impl 鍵面動態着色法 for 功能鍵開關狀態 {
    fn 鍵位提示(&self, _鍵: &鍵的定義) -> bool {
        false
    }

    fn 是否落鍵(&self, 鍵: &鍵的定義) -> bool {
        match 鍵.鍵碼 {
            KeyCode::Enter => (self.實時工作模式)() == 工作模式::輸入反查碼,
            KeyCode::Escape => (self.實時工作模式)() == 工作模式::選取練習題,
            _ => false,
        }
    }

    fn 是否擊中(&self, _鍵: &鍵的定義) -> bool {
        false
    }
}

struct 字幕指標<'a> {
    字幕: &'a str,
    指標: usize,
}

impl<'a> From<&'a str> for 字幕指標<'a> {
    fn from(字幕: &'a str) -> Self {
        Self { 字幕, 指標: 0 }
    }
}

/// 迭代字幕中的文字.
/// 傳入的字幕應當是從空白處切分出的一段.
/// 通常一音對一字. 例外情況用文字組標記 `[]` 括住與一個音節對應的一組文字.
/// 文字組不能包含空白字符及左右方括號.
impl<'a> Iterator for 字幕指標<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut 剩餘文字 = self.字幕.chars().skip(self.指標);
        match 剩餘文字.next() {
            Some('[') => {
                // 將文字組標記 [] 中的文字串視作一個文字
                let 文字組 = 剩餘文字.take_while(|字| *字 != ']');
                self.指標 += 文字組.clone().count() + 2;
                Some(文字組.collect())
            }
            Some(單字) => {
                self.指標 += 1;
                Some(單字.to_string())
            }
            None => None,
        }
    }
}

#[derive(Clone, PartialEq)]
enum 工作模式 {
    錄入,
    輸入反查碼,
    選取練習題,
}

#[derive(Clone, PartialEq)]
pub struct 作業 {
    pub 題號: Option<usize>,
    pub 自訂反查碼: Option<String>,
}

impl 作業 {
    pub fn 練習題(題號: usize) -> Self {
        Self {
            題號: Some(題號),
            自訂反查碼: None,
        }
    }
    pub fn 自訂(反查碼: String) -> Self {
        Self {
            題號: None,
            自訂反查碼: Some(反查碼),
        }
    }
    pub fn 自習() -> Self {
        Self {
            題號: None,
            自訂反查碼: None,
        }
    }

    pub fn 反查碼(&self) -> &str {
        self.題號
            .and_then(|題號| 預設練習題.get(題號).map(|題| 題.編碼))
            .or(self.自訂反查碼.as_deref())
            .unwrap_or("")
    }
    pub fn 字幕(&self) -> Option<&'static str> {
        self.題號
            .and_then(|題號| 預設練習題.get(題號))
            .and_then(|題| 題.字幕)
    }
}

#[component]
fn RIME_鍵圖<T>(
    鍵: &'static 鍵的定義,
    #[prop(optional)] 目標盤面: 盤面選擇碼,
    着色法: T,
) -> impl IntoView
where
    T: 鍵面動態着色法 + Copy + 'static,
{
    let 標註法 = 鍵面標註依據 { 鍵, 目標盤面 };
    view! {
        <div class="key"
            class:empty={move || 標註法.是否空鍵()}
            class:fallback={move || 標註法.是否後備盤面()}
            class:function={move || 標註法.是否功能鍵()}
            class:space={move || 標註法.是否空格()}
            class:hint={move || 着色法.鍵位提示(鍵)}
            class:keydown={move || 着色法.是否落鍵(鍵)}
            class:pressed={move || 着色法.是否擊中(鍵)}
        >
            <kbd class="label">{move || 標註法.刻印()}</kbd>
        </div>
    }
}

#[component]
fn RIME_鍵盤圖<T>(目標盤面: 盤面選擇碼, 着色法: T) -> impl IntoView
where
    T: 鍵面動態着色法 + Copy + 'static,
{
    view! {
        <div class="board">
        { 鍵盤矩陣.iter().map(|行| view! {
            <div class="row">
            { 行.iter().map(|鍵| {
                view! {
                    <RIME_鍵圖 鍵={鍵} 目標盤面={目標盤面} 着色法={着色法}/>
                }
            }).collect_view() }
            </div>
        }).collect_view() }
        </div>
    }
}

#[component]
pub fn RIME_打字機應用() -> impl IntoView {
    let (並擊狀態流, 並擊狀態變更) = create_signal(並擊狀態::new());
    let 實況並擊碼 = move || 並擊狀態流.with(並擊狀態::並擊序列);
    let 並擊所得拼音 = create_memo(move |_| 並擊狀態::並擊變換(&實況並擊碼()));

    let 重置並擊狀態 = move || 並擊狀態變更.update(並擊狀態::重置);

    let 鍵盤輸入焦點源 = create_selector(use_window_focus());
    create_effect(move |_| {
        if 鍵盤輸入焦點源.selected(false) {
            重置並擊狀態();
        }
    });

    let (實時工作模式, 設置工作模式) = create_signal(工作模式::錄入);
    let (當前作業, 佈置作業) = create_signal(作業::練習題(0));

    let 反查拼音組 =
        create_memo(move |_| 當前作業.with(|作業| 解析輸入碼序列(作業.反查碼())));
    let (反查進度, 更新反查進度) = create_signal(0);

    let 反查推進 = move |迴轉: bool| {
        let 拼音數 = 反查拼音組.with(Vec::len);
        if 迴轉 && 反查進度() + 1 >= 拼音數 {
            更新反查進度(0);
            return true;
        }
        // 非迴轉態可推進至結束位置，即拼音數
        if 反查進度() < 拼音數 {
            更新反查進度(反查進度() + 1);
            return true;
        }
        false
    };
    let 反查回退 = move || {
        if 反查進度() > 0 && !反查拼音組.with(Vec::is_empty) {
            更新反查進度(反查進度() - 1);
            return true;
        }
        false
    };

    let _ = watch(
        反查拼音組,
        move |_, _, _| {
            更新反查進度(0);
        },
        false,
    );

    let 分段字幕 = create_memo(move |_| {
        當前作業.with(|作業| {
            作業.字幕().map(move |有字幕| {
                有字幕
                    .split_whitespace()
                    .fold(
                        (0, Box::new(vec![])),
                        |(起始字序, mut 已標註字序的段落), 又一段| {
                            let 結束字序 = 起始字序 + 字幕指標::from(又一段).count();
                            (*已標註字序的段落).push((起始字序, 結束字序, 又一段));
                            (結束字序, 已標註字序的段落)
                        },
                    )
                    .1
            })
        })
    });

    let 該段字幕按進度顯示 = move || {
        分段字幕.with(|有冇分段字幕| {
            有冇分段字幕.as_ref().and_then(|衆段落| {
                let 全文進度 = 反查進度();
                let 當前段落號 =
                    衆段落.partition_point(|(_, 段落結束, _)| *段落結束 <= 全文進度);
                衆段落.get(當前段落號).map(|當前段落| {
                    let (段落起始, _, 段落文字) = 當前段落;
                    let 段落進度 = 全文進度 - 段落起始;
                    let 完成的字 = 字幕指標::from(*段落文字).take(段落進度).collect::<String>();
                    let 當下的字 = 字幕指標::from(*段落文字)
                        .skip(段落進度)
                        .take(1)
                        .collect::<String>();
                    let 剩餘的字 = 字幕指標::from(*段落文字)
                        .skip(段落進度 + 1)
                        .collect::<String>();
                    (完成的字, 當下的字, 剩餘的字)
                })
            })
        })
    };

    let 目標反查輸入碼 = move || {
        反查拼音組.with(|拼音組| {
            if 拼音組.is_empty() {
                None
            } else {
                拼音組.get(min(反查進度(), 拼音組.len() - 1)).cloned()
            }
        })
    };

    let 反查鍵位 = create_memo(move |_| 目標反查輸入碼().as_ref().and_then(輸入碼::反查鍵位));
    let 反查所得並擊碼 = move || 反查鍵位().as_ref().map(鍵組::寫成並擊序列);
    let 輸入碼 = move || 反查所得並擊碼().unwrap_or_else(實況並擊碼);
    let 拼音 = move || {
        目標反查輸入碼()
            .and_then(|輸入碼| 輸入碼.轉寫碼原文)
            .or_else(|| 並擊所得拼音().to_owned())
            // 加尖括弧表示拉丁文轉寫，即拼音
            .map(|拼音| format!("⟨{拼音}⟩"))
    };

    let 顯示反查 = move || 反查鍵位().is_some();
    let 顯示實況 = move || !顯示反查() && !實況並擊碼().is_empty();
    let 並擊成功 = move || {
        // 拼音一致即爲成功，允許並擊碼不同
        目標反查輸入碼()
            .and_then(|輸入碼| 輸入碼.轉寫碼原文)
            .is_some_and(|查得| 並擊所得拼音().is_some_and(|擊得| 查得 == 擊得))
            // 拼音爲非音節形式的聲母、韻母，須比較並擊碼
            || 反查所得並擊碼().is_some_and(|查得| 查得 == 實況並擊碼())
    };
    let 並擊開始 = move || 並擊狀態流.with(|狀態| !狀態.實時落鍵.0.is_empty());
    let 並擊完成 =
        move || 並擊狀態流.with(|狀態| 狀態.實時落鍵.0.is_empty()) && !實況並擊碼().is_empty();
    let 反查進度完成 = move || 反查進度() == 反查拼音組.with(Vec::len);

    let 開啓反查輸入 = move || {
        if 反查進度完成() {
            佈置作業(作業::自習());
        }
        重置並擊狀態();
        設置工作模式(工作模式::輸入反查碼);
    };
    let 開啓練習題選單 = move || {
        更新反查進度(0);
        重置並擊狀態();
        設置工作模式(工作模式::選取練習題);
    };
    let 關閉輸入欄 = move || {
        設置工作模式(工作模式::錄入);
    };

    let _ = use_event_listener(use_document().body(), keydown, move |evt: KeyboardEvent| {
        log!("落鍵 key = {}, code = {}", &evt.key(), evt.code());
        match evt.code().as_str() {
            "Enter" => {
                if [工作模式::輸入反查碼, 工作模式::選取練習題].contains(&實時工作模式())
                {
                    關閉輸入欄();
                } else {
                    開啓反查輸入();
                }
                evt.prevent_default();
            }
            "Escape" => {
                if [工作模式::輸入反查碼, 工作模式::選取練習題].contains(&實時工作模式())
                {
                    關閉輸入欄();
                } else if 反查進度() != 0 {
                    更新反查進度(0);
                    重置並擊狀態();
                } else {
                    開啓練習題選單();
                }
                evt.prevent_default();
            }
            "Tab" => {
                if 實時工作模式() == 工作模式::輸入反查碼 {
                    關閉輸入欄();
                } else if 反查推進(true) {
                    重置並擊狀態();
                }
                evt.prevent_default();
            }
            "Backspace" => {
                if 實時工作模式() == 工作模式::錄入 {
                    if 並擊完成() || 反查回退() {
                        重置並擊狀態();
                    }
                    evt.prevent_default();
                }
            }
            _ => (),
        }
        if 實時工作模式() == 工作模式::錄入 {
            並擊狀態變更.update(|並擊| 並擊.落鍵(網頁鍵值轉換(&evt.code())));
        }
        // 繼續擊鍵時消除已完成的反查作業
        if 並擊開始() && 反查進度完成() {
            佈置作業(作業::自習());
        }
    });

    let _ = use_event_listener(use_document().body(), keyup, move |evt: KeyboardEvent| {
        log!("抬鍵 key = {}, code = {}", &evt.key(), &evt.code());
        if 實時工作模式() == 工作模式::錄入 {
            並擊狀態變更.update(|並擊| 並擊.抬鍵(網頁鍵值轉換(&evt.code())));
        }
        if 並擊完成() && 並擊成功() {
            // 擊中目標拼音後，反查下一個拼音；在最後一個拼音完成後顯示結果
            if 反查推進(false) && !反查進度完成() {
                重置並擊狀態();
            }
        }
    });

    let 反查輸入欄的引用 = create_node_ref::<html::Input>();
    create_effect(move |_| {
        if 實時工作模式() == 工作模式::輸入反查碼 {
            if let Some(輸入欄) = 反查輸入欄的引用() {
                let _不看結果 = 輸入欄.on_mount(|輸入欄| {
                    輸入欄.select();
                });
            }
        }
    });

    let 練習題選單的引用 = create_node_ref::<html::Select>();
    create_effect(move |_| {
        if 實時工作模式() == 工作模式::選取練習題 {
            let 選中題號: i32 = 當前作業
                .with(|作業| 作業.題號)
                .and_then(|題號| 題號.try_into().ok())
                .unwrap_or(-1);
            if let Some(輸入欄) = 練習題選單的引用() {
                let _不看結果 = 輸入欄.on_mount(move |輸入欄| {
                    輸入欄.set_selected_index(選中題號);
                    let _ = 輸入欄.focus();
                });
            }
        }
    });

    let 編碼欄 = view! {
         <div class="input-code"
            class:freeplay={顯示實況}
            class:target={顯示反查}
            class:success={並擊成功}
            on:click=move |_| {
                if 實時工作模式() == 工作模式::錄入 {
                    if 當前作業.with(|作業| 作業.題號).is_some() {
                        開啓練習題選單()
                    } else {
                        開啓反查輸入()
                    }
                }
            }
        >
        {
            move || match 實時工作模式() {
                工作模式::錄入 => view! {
                    <kbd class="raw-input">{輸入碼}</kbd>
                    <span class="translated-input">{拼音}</span>
                }.into_view(),
                工作模式::輸入反查碼 => view! {
                    <input type="text" class="lookup-code"
                        _ref=反查輸入欄的引用
                        placeholder="qing shu ru pin yin"
                        value=move || 當前作業.with(|作業| 作業.反查碼().to_owned())
                        on:input=move |ev| {
                            let 輸入文字 = event_target_value(&ev);
                            佈置作業(作業::自訂(輸入文字));
                        }
                        on:blur=move |_| 關閉輸入欄()
                    />
                }.into_view(),
                工作模式::選取練習題 => view! {
                    <select class="excercises"
                        _ref=練習題選單的引用
                        on:change=move |ev| {
                            let 題號 = event_target_value(&ev);
                            log!("題號: {}", 題號);
                            if let Ok(題號) = 題號.parse::<usize>() {
                                if 題號 < 預設練習題.len() {
                                    佈置作業(作業::練習題(題號));
                                    關閉輸入欄();
                                }
                            }
                        }
                        on:blur=move |_| 關閉輸入欄()
                    >
                    {
                        預設練習題.iter().enumerate().map(|(題號, 題)| view! {
                            <option value={題號}>{題.標題}</option>
                        }).collect_view()
                    }
                    </select>
                }.into_view(),
            }
        }
        </div>
    };

    let 動態 = 擊鍵動態 {
        目標並擊: 反查鍵位.into(),
        實況並擊: 並擊狀態流.into(),
    };

    let 各開關狀態 = 功能鍵開關狀態 {
        實時工作模式: Signal::derive(實時工作模式),
    };

    let styler_class = 樣式();
    view! { class = styler_class,
        <div class="text-box">
            <div class="caption">
            {
                move || 該段字幕按進度顯示().map(|(完成的字, 當下的字, 剩餘的字)| view! {
                    <span class="accepted">{完成的字}</span>
                    <span class="highlight">{當下的字}</span>
                    <span>{剩餘的字}</span>
                })
            }
            </div>
        </div>
        <div class="echo-bar">
            <div title="重新錄入／重選練習題">
                <RIME_鍵圖 鍵={&退出鍵} 着色法={各開關狀態}/>
            </div>
            <div title="前進一字">
                <RIME_鍵圖 鍵={&製表鍵} 着色法={動態}/>
            </div>
            <div class="function key hidden"/>
            {編碼欄}
            <div class="function key hidden"/>
            <div title="刪除／回退一字">
                <RIME_鍵圖 鍵={&退格鍵} 着色法={動態}/>
            </div>
            <div title="輸入拼音反查鍵位">
                <RIME_鍵圖 鍵={&回車鍵} 着色法={各開關狀態}/>
            </div>
        </div>
        <RIME_鍵盤圖 目標盤面={宮保拼音盤面} 着色法={動態}/>
    }
}
