use keyberon::key_code::KeyCode;
use leptos::*;

use crate::assignment::{作業, 作業機關};
use crate::chord::並擊機關;
use crate::engine::{並擊狀態, 鍵組};
use crate::input::{焦點事件處理機關, 輸入事件處理機關};
use crate::layout::{
    功能鍵::{回車鍵, 製表鍵, 退出鍵, 退格鍵},
    盤面選擇碼, 鍵的定義,
};
use crate::mode::{工作模式, 工作模式機關};
use crate::style::樣式;
use crate::view::{
    caption::Rime字幕屏,
    input_code::{
        Rime反查輸入欄, Rime編碼回顯區, Rime編碼欄, Rime練習題選單, 編碼欄顯示選項
    },
    keyboard::{Rime鍵圖, Rime鍵盤圖, 鍵面動態着色法},
};

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
    let (當前作業, 佈置作業, 作業進度, 作業進度完成, 目標輸入碼, 重置作業進度, 作業推進, 作業回退) =
        作業機關();

    let (
        並擊狀態流,
        並擊狀態變更,
        實況並擊碼,
        並擊所得拼音,
        反查鍵位,
        反查所得並擊碼,
        並擊開始,
        並擊完成,
        並擊成功,
        重置並擊狀態,
    ) = 並擊機關(目標輸入碼);

    let (現行工作模式, 開啓反查輸入, 開啓練習題選單, 關閉輸入欄) =
        工作模式機關(作業進度完成, 佈置作業, 重置作業進度, 重置並擊狀態);

    焦點事件處理機關(重置並擊狀態);

    let 處理退出鍵 = move || {
        if 現行工作模式() == 工作模式::錄入 {
            if 作業進度() != 0 {
                重置作業進度();
                重置並擊狀態();
            } else {
                開啓練習題選單();
            }
        } else {
            關閉輸入欄();
        }
        true
    };
    let 處理製表鍵 = move || {
        if 現行工作模式() == 工作模式::錄入 {
            if 作業推進(true).is_ok() {
                重置並擊狀態();
            }
        } else {
            關閉輸入欄();
        }
        true
    };
    let 處理退格鍵 = move || {
        if 現行工作模式() == 工作模式::錄入 {
            if 並擊完成() || 作業回退().is_ok() {
                重置並擊狀態();
            }
            return true;
        }
        false
    };
    let 處理回車鍵 = move || {
        if 現行工作模式() == 工作模式::錄入 {
            開啓反查輸入();
        } else {
            關閉輸入欄();
        }
        true
    };
    let 既然落鍵 = move || {
        // 繼續擊鍵時消除已完成的反查作業
        if 並擊開始() && 作業進度完成() {
            佈置作業(作業::自習());
        }
    };
    let 既然抬鍵 = move || {
        if 並擊完成() && 並擊成功() {
            // 擊中目標拼音後，反查下一個拼音；在最後一個拼音完成後顯示結果
            if 作業推進(false).is_ok() && !作業進度完成() {
                重置並擊狀態();
            }
        }
    };

    輸入事件處理機關(
        並擊狀態變更,
        現行工作模式,
        處理退出鍵,
        處理製表鍵,
        處理退格鍵,
        處理回車鍵,
        既然落鍵,
        既然抬鍵,
    );

    // 界面

    let 顯示選項 = Signal::derive(move || {
        if 反查鍵位().is_some() {
            編碼欄顯示選項::顯示反查
        } else if !實況並擊碼().is_empty() {
            編碼欄顯示選項::顯示實況
        } else {
            編碼欄顯示選項::無顯示
        }
    });
    let 顯示輸入碼 = Signal::derive(move || 反查所得並擊碼().unwrap_or_else(實況並擊碼));
    let 顯示拼音 = Signal::derive(move || {
        目標輸入碼()
            .and_then(|輸入碼| 輸入碼.轉寫碼原文)
            .or_else(|| 並擊所得拼音().to_owned())
            // 加尖括弧表示拉丁文轉寫，即拼音
            .map(|拼音| format!("⟨{拼音}⟩"))
    });
    let 反查碼 = Signal::derive(move || 當前作業.with(|作業| 作業.反查碼().to_owned()));
    let 當選題號 = Signal::derive(move || 當前作業.with(|作業| 作業.題號));

    let 動態 = 擊鍵動態 {
        目標並擊: 反查鍵位,
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
                顯示選項={顯示選項}
                並擊成功={並擊成功}
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
                        <Rime編碼回顯區 輸入碼={顯示輸入碼} 拼音={顯示拼音}/>
                    }.into_view(),
                    工作模式::輸入反查碼 => view! {
                        <Rime反查輸入欄
                            反查碼={反查碼}
                            示例輸入={Signal::derive(|| String::from("qing shu ru pin yin"))}
                            反查碼變更=move |反查碼| {
                                佈置作業(作業::自訂(反查碼));
                            }
                            關閉輸入欄={關閉輸入欄}
                        />
                    }.into_view(),
                    工作模式::選取練習題 => view! {
                        <Rime練習題選單
                            當選題號={當選題號}
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
