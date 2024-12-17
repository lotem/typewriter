use keyberon::key_code::KeyCode;
use leptos::prelude::*;

use crate::definition::{觸鍵方式, 鍵組};
use crate::engine::微觀引擎;
use crate::gear::{assignment::作業, chord::並擊狀態, mode::工作模式};
use crate::layout::功能鍵::{
    回車鍵, 撇號鍵, 製表鍵, 退出鍵, 退格鍵, 重音符鍵
};
use crate::view::{
    caption::Rime字幕屏,
    exercise_menu::Rime練習題選單,
    input_code::{
        Rime反查輸入欄, Rime編碼回顯區, Rime編碼欄, 回顯區佈局, 編碼欄顯示選項
    },
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
            .read()
            .as_ref()
            .is_some_and(|並擊| 並擊.0.contains(&鍵))
    }

    fn 是否落鍵(&self, 鍵: KeyCode) -> bool {
        self.實況並擊.read().實時落鍵.0.contains(&鍵)
    }

    fn 是否擊中(&self, 鍵: KeyCode) -> bool {
        self.實況並擊.read().累計擊鍵.0.contains(&鍵)
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
    let (
        反查鍵位,
        有無輸入碼,
        指法,
        並擊狀態流,
        現行工作模式,
        字幕段落表示,
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
    ) = 微觀引擎();

    let 是否顯示光標 = Signal::derive(move || matches!(指法(), 觸鍵方式::連擊));
    let 顯示選項 = Signal::derive(move || {
        if 反查鍵位().is_some() {
            編碼欄顯示選項::顯示反查
        } else if 有無輸入碼() {
            編碼欄顯示選項::顯示實況
        } else {
            編碼欄顯示選項::無顯示
        }
    });
    let 編碼回顯區佈局 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => 回顯區佈局::單欄,
        觸鍵方式::並擊 => 回顯區佈局::左右對照,
    });
    let 反查碼 = Signal::derive(move || 當前作業.read().反查碼().map(str::to_owned));
    let 當選題號 = Signal::derive(move || 當前作業.read().題號);
    let 方案配套練習題 = Signal::derive(move || 現行方案().配套練習題().unwrap_or(&[]));
    let 方案指定盤面 = Signal::derive(move || 方案定義.read().盤面);

    let 標註功能鍵 = |功能鍵| Signal::derive(move || 功能鍵);

    let 並擊動態 = 並擊對標動態 {
        目標並擊: 反查鍵位,
        實況並擊: 並擊狀態流.into(),
    };

    let 開關狀態 = 功能鍵開關狀態 { 現行工作模式 };

    view! {
        <Rime字幕屏 是否顯示光標={是否顯示光標} 按進度顯示字幕={字幕段落表示}/>
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
                輸入正確={輸入正確}
                點擊動作=move || {
                    if 現行工作模式() == 工作模式::錄入 {
                        if 當前作業.read().是否練習題() {
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
                        <Rime編碼回顯區 佈局={編碼回顯區佈局} 輸入碼={回顯輸入碼} 轉寫碼={回顯轉寫碼}/>
                    }.into_any(),
                    工作模式::輸入反查碼 => view! {
                        <Rime反查輸入欄
                            反查碼={反查碼}
                            示例輸入={Signal::derive(|| String::from("qing shu ru pin yin"))}
                            反查碼變更=move |反查碼| {
                                佈置作業(作業::自訂(現行方案(), 反查碼));
                            }
                        />
                    }.into_any(),
                    工作模式::選取練習題 => view! {
                        <Rime練習題選單
                            預設練習題={方案配套練習題}
                            當選題號={當選題號}
                            選中題號=move |題號| {
                                佈置作業(作業::練習題(現行方案(), 題號));
                                關閉輸入欄();
                            }
                        />
                    }.into_any(),
                    工作模式::選擇輸入方案 => view! {
                        <Rime方案選單
                            現行方案={現行方案}
                            選中方案=move |方案| {
                                選用方案(方案);
                                關閉輸入欄();
                            }
                        />
                    }.into_any(),
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
