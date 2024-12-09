use keyberon::key_code::KeyCode;
use leptos::logging::log;
use leptos::*;

use crate::assignment::{作業, 作業推進參數, 作業機關};
use crate::caption::{字幕機關, 字幕段落};
use crate::chord::並擊機關;
use crate::engine::{並擊狀態, 觸鍵方式, 鍵組};
use crate::input::{焦點事件處理機關, 輸入事件處理機關};
use crate::key_press::連擊機關;
use crate::layout::功能鍵::{
    回車鍵, 撇號鍵, 製表鍵, 退出鍵, 退格鍵, 重音符鍵
};
use crate::mode::{工作模式, 工作模式機關};
use crate::style::樣式;
use crate::theory::輸入方案機關;
use crate::view::{
    caption::Rime字幕屏,
    exercise_menu::Rime練習題選單,
    input_code::{Rime反查輸入欄, Rime編碼回顯區, Rime編碼欄, 編碼欄顯示選項},
    keyboard::{Rime鍵圖, Rime鍵盤圖, 鍵面動態着色法},
    theory_menu::Rime方案選單,
};

#[derive(Clone, Copy)]
struct 並擊對標動態 {
    目標並擊: Signal<Option<鍵組>>,
    實況並擊: Signal<並擊狀態>,
}

impl 鍵面動態着色法 for 並擊對標動態 {
    fn 鍵位提示(&self, 鍵: KeyCode) -> bool {
        self.目標並擊
            .with(|有冇| 有冇.as_ref().is_some_and(|並擊| 並擊.0.contains(&鍵)))
    }

    fn 是否落鍵(&self, 鍵: KeyCode) -> bool {
        self.實況並擊.with(|並擊| 並擊.實時落鍵.0.contains(&鍵))
    }

    fn 是否擊中(&self, 鍵: KeyCode) -> bool {
        self.實況並擊.with(|並擊| 並擊.累計擊鍵.0.contains(&鍵))
    }
}

#[derive(Clone, Copy)]
struct 功能鍵開關狀態 {
    現行工作模式: ReadSignal<工作模式>,
}

impl 鍵面動態着色法 for 功能鍵開關狀態 {
    fn 鍵位提示(&self, _鍵: KeyCode) -> bool {
        false
    }

    fn 是否落鍵(&self, 鍵: KeyCode) -> bool {
        match 鍵 {
            KeyCode::Enter => self.現行工作模式.get() == 工作模式::輸入反查碼,
            KeyCode::Escape => self.現行工作模式.get() == 工作模式::選取練習題,
            KeyCode::Grave => self.現行工作模式.get() == 工作模式::選擇輸入方案,
            _ => false,
        }
    }

    fn 是否擊中(&self, _鍵: KeyCode) -> bool {
        false
    }
}

#[component]
pub fn Rime打字機應用() -> impl IntoView {
    let (現行方案, 選用方案, 方案定義) = 輸入方案機關();

    let 指法 = Signal::derive(move || 方案定義.with(|方案| 方案.指法));

    let (
        當前作業,
        佈置作業,
        作業進度,
        作業進度完成,
        輸入碼序列,
        目標輸入碼,
        重置作業進度,
        作業推進,
        作業回退,
    ) = 作業機關(現行方案, 方案定義);

    let (分段字幕, 當前段落, 按進度顯示字幕段落) =
        字幕機關(當前作業, 作業進度, 輸入碼序列, 指法);

    let (連擊狀態流, 連擊狀態變更, 實況字根碼, 反查所得字根碼, 連擊比對成功, 重置連擊狀態) =
        連擊機關(方案定義, 目標輸入碼);

    let (
        並擊狀態流,
        並擊狀態變更,
        實況並擊碼,
        並擊所得拼音,
        反查所得並擊碼,
        反查鍵位,
        並擊開始,
        並擊完成,
        並擊成功,
        重置並擊狀態,
    ) = 並擊機關(方案定義, 目標輸入碼);

    let (現行工作模式, 開啓反查輸入, 開啓練習題選單, 開啓方案選單, 關閉輸入欄) =
        工作模式機關(現行方案, 作業進度完成, 佈置作業, 重置作業進度, 重置並擊狀態);

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
                        重置並擊狀態();
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
                    重置並擊狀態();
                }
            } else {
                關閉輸入欄();
            }
            true
        }
        KeyCode::BSpace => {
            if 現行工作模式() == 工作模式::錄入 {
                if 並擊完成() || 作業回退().is_ok() {
                    重置並擊狀態();
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
    let 既然落鍵 = move || {
        // 繼續擊鍵時消除已完成的反查作業
        if 並擊開始() && 作業進度完成() {
            佈置作業(作業::自習(現行方案()));
        }
        if 指法() == 觸鍵方式::連擊 {
            批閱作業();
        }
    };
    let 既然抬鍵 = move || {
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

    輸入事件處理機關(
        連擊狀態變更,
        並擊狀態變更,
        現行工作模式,
        處理功能鍵,
        既然落鍵,
        既然抬鍵,
    );

    // 界面

    let 有無輸入碼 = move || match 指法() {
        觸鍵方式::連擊 => !實況字根碼().is_empty(),
        觸鍵方式::並擊 => !實況並擊碼().is_empty(),
    };
    let 顯示選項 = Signal::derive(move || {
        if 反查鍵位().is_some() {
            編碼欄顯示選項::顯示反查
        } else if 有無輸入碼() {
            編碼欄顯示選項::顯示實況
        } else {
            編碼欄顯示選項::無顯示
        }
    });
    let 擊中目標 = Signal::derive(擊中目標);
    let 顯示輸入碼 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => 反查所得字根碼().unwrap_or_else(實況字根碼),
        觸鍵方式::並擊 => 反查所得並擊碼().unwrap_or_else(實況並擊碼),
    });
    let 顯示轉寫碼 = Signal::derive(move || {
        目標輸入碼()
            .and_then(|輸入碼| 輸入碼.轉寫碼原文)
            .or_else(|| {
                if 指法() == 觸鍵方式::並擊 {
                    並擊所得拼音().to_owned()
                } else {
                    None
                }
            })
            // 加尖括弧表示拉丁文轉寫
            .map(|轉寫| format!("⟨{轉寫}⟩"))
    });
    let 反查碼 = Signal::derive(move || 當前作業.with(|作業| 作業.反查碼().to_owned()));
    let 當選題號 = Signal::derive(move || 當前作業.with(|作業| 作業.題號));
    let 方案配套練習題 = Signal::derive(move || 現行方案().配套練習題().unwrap_or(&[]));

    let 方案指定盤面 = Signal::derive(move || 方案定義.with(|方案| 方案.盤面));

    let 標註功能鍵 = |功能鍵| Signal::derive(move || 功能鍵);

    let 並擊動態 = 並擊對標動態 {
        目標並擊: 反查鍵位,
        實況並擊: 並擊狀態流.into(),
    };

    let 開關狀態 = 功能鍵開關狀態 { 現行工作模式 };

    let styler_class = 樣式();
    view! { class = styler_class,
        <Rime字幕屏 按進度顯示字幕={按進度顯示字幕段落}/>
        <div class="echo-bar">
            <div title="選輸入方案">
                <Rime鍵圖 鍵={重音符鍵.鍵碼} 標註法={標註功能鍵(重音符鍵)} 着色法={開關狀態}/>
            </div>
            <div title="重新錄入／選練習題">
                <Rime鍵圖 鍵={退出鍵.鍵碼} 標註法={標註功能鍵(退出鍵)} 着色法={開關狀態}/>
            </div>
            <div title="下一題">
                <Rime鍵圖 鍵={製表鍵.鍵碼} 標註法={標註功能鍵(製表鍵)} 着色法={並擊動態}/>
            </div>
            <Rime編碼欄
                顯示選項={顯示選項}
                擊中目標={擊中目標}
                點擊動作=move || {
                    if 現行工作模式() == 工作模式::錄入 {
                        if 當前作業.with(作業::是否練習題) {
                            開啓練習題選單();
                        } else {
                            開啓反查輸入();
                        }
                    }
                }
                關閉輸入欄={關閉輸入欄}
            >
            {
                move || match 現行工作模式() {
                    工作模式::錄入 => view! {
                        <Rime編碼回顯區 輸入碼={顯示輸入碼} 轉寫碼={顯示轉寫碼}/>
                    }.into_view(),
                    工作模式::輸入反查碼 => view! {
                        <Rime反查輸入欄
                            反查碼={反查碼}
                            示例輸入={Signal::derive(|| String::from("qing shu ru pin yin"))}
                            反查碼變更=move |反查碼| {
                                佈置作業(作業::自訂(現行方案(), 反查碼));
                            }
                        />
                    }.into_view(),
                    工作模式::選取練習題 => view! {
                        <Rime練習題選單
                            預設練習題={方案配套練習題}
                            當選題號={當選題號}
                            選中題號=move |題號| {
                                佈置作業(作業::練習題(現行方案(), 題號));
                                關閉輸入欄();
                            }
                        />
                    }.into_view(),
                    工作模式::選擇輸入方案 => view! {
                        <Rime方案選單
                            現行方案={現行方案}
                            選中方案=move |方案| {
                                選用方案(方案);
                                關閉輸入欄();
                            }
                        />
                    }.into_view(),
                }
            }
            </Rime編碼欄>
            <div title="撇號">
                <Rime鍵圖 鍵={撇號鍵.鍵碼} 標註法={標註功能鍵(撇號鍵)} 着色法={並擊動態}/>
            </div>
            <div title="輸入拼音反查鍵位">
                <Rime鍵圖 鍵={回車鍵.鍵碼} 標註法={標註功能鍵(回車鍵)} 着色法={開關狀態}/>
            </div>
            <div title="刪除／回退一字">
                <Rime鍵圖 鍵={退格鍵.鍵碼} 標註法={標註功能鍵(退格鍵)} 着色法={並擊動態}/>
            </div>
        </div>
        <Rime鍵盤圖 目標盤面={方案指定盤面} 着色法={並擊動態}/>
    }
}
