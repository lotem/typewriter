use keyberon::key_code::KeyCode;
use leptos::*;
use leptos::{
    ev::{keydown, keyup, KeyboardEvent},
    logging::log,
};
use leptos_use::{use_document, use_event_listener, use_window_focus};
use std::cmp::min;

use crate::assignment::作業;
use crate::engine::{並擊狀態, 解析輸入碼序列, 輸入碼, 鍵組};
use crate::key_code::網頁鍵值轉換;
use crate::layout::{
    功能鍵::{回車鍵, 製表鍵, 退出鍵, 退格鍵},
    盤面選擇碼, 鍵的定義,
};
use crate::style::樣式;
use crate::view::{
    caption::Rime字幕屏,
    input_code::{
        Rime反查輸入欄, Rime編碼回顯區, Rime編碼欄, Rime練習題選單, 編碼欄顯示選項
    },
    keyboard::{Rime鍵圖, Rime鍵盤圖, 鍵面動態着色法},
};

#[derive(Clone, PartialEq)]
enum 工作模式 {
    錄入,
    輸入反查碼,
    選取練習題,
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
    現行工作模式: ReadSignal<工作模式>,
}

impl 鍵面動態着色法 for 功能鍵開關狀態 {
    fn 鍵位提示(&self, _鍵: &鍵的定義) -> bool {
        false
    }

    fn 是否落鍵(&self, 鍵: &鍵的定義) -> bool {
        match 鍵.鍵碼 {
            KeyCode::Enter => self.現行工作模式.get() == 工作模式::輸入反查碼,
            KeyCode::Escape => self.現行工作模式.get() == 工作模式::選取練習題,
            _ => false,
        }
    }

    fn 是否擊中(&self, _鍵: &鍵的定義) -> bool {
        false
    }
}

const 默認盤面: 盤面選擇碼 = 盤面選擇碼(0);
const 選擇宮保拼音盤面: 盤面選擇碼 = 盤面選擇碼(2);

#[component]
pub fn Rime打字機應用() -> impl IntoView {
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

    let (現行工作模式, 設置工作模式) = create_signal(工作模式::錄入);
    let (當前作業, 佈置作業) = create_signal(作業::練習題(0));

    let 反查拼音組 =
        create_memo(move |_| 當前作業.with(|作業| 解析輸入碼序列(作業.反查碼())));
    let (作業進度, 更新作業進度) = create_signal(0);

    let 作業推進 = move |迴轉: bool| {
        let 拼音數 = 反查拼音組.with(Vec::len);
        if 迴轉 && 作業進度() + 1 >= 拼音數 {
            更新作業進度(0);
            return true;
        }
        // 非迴轉態可推進至結束位置，即拼音數
        if 作業進度() < 拼音數 {
            更新作業進度(作業進度() + 1);
            return true;
        }
        false
    };
    let 作業回退 = move || {
        if 作業進度() > 0 && !反查拼音組.with(Vec::is_empty) {
            更新作業進度(作業進度() - 1);
            return true;
        }
        false
    };

    let _ = watch(
        反查拼音組,
        move |_, _, _| {
            更新作業進度(0);
        },
        false,
    );

    let 目標反查輸入碼 = move || {
        反查拼音組.with(|拼音組| {
            if 拼音組.is_empty() {
                None
            } else {
                拼音組.get(min(作業進度(), 拼音組.len() - 1)).cloned()
            }
        })
    };

    let 反查鍵位 = create_memo(move |_| 目標反查輸入碼().as_ref().and_then(輸入碼::反查鍵位));
    let 反查所得並擊碼 = move || 反查鍵位().as_ref().map(鍵組::寫成並擊序列);

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
    let 作業進度完成 = move || 作業進度() == 反查拼音組.with(Vec::len);

    let 開啓反查輸入 = move || {
        if 作業進度完成() {
            佈置作業(作業::自習());
        }
        重置並擊狀態();
        設置工作模式(工作模式::輸入反查碼);
    };
    let 開啓練習題選單 = move || {
        更新作業進度(0);
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
                if 現行工作模式() == 工作模式::錄入 {
                    開啓反查輸入();
                } else {
                    關閉輸入欄();
                }
                evt.prevent_default();
            }
            "Escape" => {
                if 現行工作模式() == 工作模式::錄入 {
                    if 作業進度() != 0 {
                        更新作業進度(0);
                        重置並擊狀態();
                    } else {
                        開啓練習題選單();
                    }
                } else {
                    關閉輸入欄();
                }
                evt.prevent_default();
            }
            "Tab" => {
                if 現行工作模式() == 工作模式::錄入 {
                    if 作業推進(true) {
                        重置並擊狀態();
                    }
                } else {
                    關閉輸入欄();
                }
                evt.prevent_default();
            }
            "Backspace" => {
                if 現行工作模式() == 工作模式::錄入 {
                    if 並擊完成() || 作業回退() {
                        重置並擊狀態();
                    }
                    evt.prevent_default();
                }
            }
            _ => (),
        }
        if 現行工作模式() == 工作模式::錄入 {
            並擊狀態變更.update(|並擊| 並擊.落鍵(網頁鍵值轉換(&evt.code())));
        }
        // 繼續擊鍵時消除已完成的反查作業
        if 並擊開始() && 作業進度完成() {
            佈置作業(作業::自習());
        }
    });

    let _ = use_event_listener(use_document().body(), keyup, move |evt: KeyboardEvent| {
        log!("抬鍵 key = {}, code = {}", &evt.key(), &evt.code());
        if 現行工作模式() == 工作模式::錄入 {
            並擊狀態變更.update(|並擊| 並擊.抬鍵(網頁鍵值轉換(&evt.code())));
        }
        if 並擊完成() && 並擊成功() {
            // 擊中目標拼音後，反查下一個拼音；在最後一個拼音完成後顯示結果
            if 作業推進(false) && !作業進度完成() {
                重置並擊狀態();
            }
        }
    });

    let 顯示選項 = move || {
        if 反查鍵位().is_some() {
            編碼欄顯示選項::顯示反查
        } else if !實況並擊碼().is_empty() {
            編碼欄顯示選項::顯示實況
        } else {
            編碼欄顯示選項::無顯示
        }
    };
    let 顯示輸入碼 = move || 反查所得並擊碼().unwrap_or_else(實況並擊碼);
    let 顯示拼音 = move || {
        目標反查輸入碼()
            .and_then(|輸入碼| 輸入碼.轉寫碼原文)
            .or_else(|| 並擊所得拼音().to_owned())
            // 加尖括弧表示拉丁文轉寫，即拼音
            .map(|拼音| format!("⟨{拼音}⟩"))
    };

    let 反查碼 = move || 當前作業.with(|作業| 作業.反查碼().to_owned());
    let 當選題號 = move || 當前作業.with(|作業| 作業.題號);

    let 動態 = 擊鍵動態 {
        目標並擊: 反查鍵位.into(),
        實況並擊: 並擊狀態流.into(),
    };

    let 開關狀態 = 功能鍵開關狀態 { 現行工作模式 };

    let styler_class = 樣式();
    view! { class = styler_class,
        <Rime字幕屏 當前作業={當前作業.into()} 作業進度={作業進度.into()}/>
        <div class="echo-bar">
            <div title="重新錄入／重選練習題">
                <Rime鍵圖 鍵={&退出鍵} 標註法={默認盤面} 着色法={開關狀態}/>
            </div>
            <div title="前進一字">
                <Rime鍵圖 鍵={&製表鍵} 標註法={默認盤面} 着色法={動態}/>
            </div>
            <div class="function key hidden"/>
            <Rime編碼欄
                顯示選項={Signal::derive(顯示選項)}
                並擊成功={Signal::derive(並擊成功)}
                點擊動作=move || {
                    if 現行工作模式() == 工作模式::錄入 {
                        if 當前作業.with(作業::是否練習題) {
                            開啓練習題選單();
                        } else {
                            開啓反查輸入();
                        }
                    }
                }
            >
            {
                move || match 現行工作模式() {
                    工作模式::錄入 => view! {
                        <Rime編碼回顯區 輸入碼={Signal::derive(顯示輸入碼)} 拼音={Signal::derive(顯示拼音)}/>
                    }.into_view(),
                    工作模式::輸入反查碼 => view! {
                        <Rime反查輸入欄
                            反查碼={Signal::derive(反查碼)}
                            示例輸入={Signal::derive(|| String::from("qing shu ru pin yin"))}
                            反查碼變更=move |反查碼| {
                                佈置作業(作業::自訂(反查碼));
                            }
                            關閉輸入欄={關閉輸入欄}
                        />
                    }.into_view(),
                    工作模式::選取練習題 => view! {
                        <Rime練習題選單
                            當選題號={Signal::derive(當選題號)}
                            選中題號=move |題號| {
                                佈置作業(作業::練習題(題號));
                                關閉輸入欄();
                            }
                            關閉選單={關閉輸入欄}
                        />
                    }.into_view(),
                }
            }
            </Rime編碼欄>
            <div class="function key hidden"/>
            <div title="刪除／回退一字">
                <Rime鍵圖 鍵={&退格鍵} 標註法={默認盤面} 着色法={動態}/>
            </div>
            <div title="輸入拼音反查鍵位">
                <Rime鍵圖 鍵={&回車鍵} 標註法={默認盤面} 着色法={開關狀態}/>
            </div>
        </div>
        <Rime鍵盤圖 標註法={選擇宮保拼音盤面} 着色法={動態}/>
    }
}
