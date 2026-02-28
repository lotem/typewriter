use leptos::prelude::*;

use crate::definition::{觸鍵方式, 鍵組};
use crate::engine::{微觀引擎, 微觀引擎輸出信號};
use crate::gear::{
    assignment::{作業, 作業機關輸出信號},
    caption::字幕機關輸出信號,
    chord::{並擊機關輸出信號, 並擊狀態},
    key_press::連擊機關輸出信號,
    layout::{
        佈局機關輸出信號,
        功能鍵::{回車鍵, 製表鍵, 退出鍵, 退格鍵},
    },
    mode::{工作模式, 工作模式機關輸出信號},
    theory::輸入方案機關輸出信號,
};
use crate::key_code::KeyCode;
use crate::view::{
    caption::Rime字幕屏,
    exercise_menu::Rime練習題選單,
    input_code::{
        Rime反查輸入欄, Rime編碼回顯區, Rime編碼欄, 回顯區佈局, 編碼欄顯示選項
    },
    keyboard::{Rime鍵圖, Rime鍵盤圖, 鍵面動態着色法},
    layout_menu::Rime配列選單,
    status_bar::Rime狀態欄,
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
    let 微觀引擎輸出信號 {
        方案,
        模式,
        佈局,
        作業,
        字幕,
        連擊,
        並擊,
    } = 微觀引擎();
    let 輸入方案機關輸出信號 {
        現行方案,
        選用方案,
        方案定義,
        指法,
        ..
    } = 方案;
    let 佈局機關輸出信號 {
        實際配列,
        選用配列,
        當選盤面,
        ..
    } = 佈局;
    let 工作模式機關輸出信號 {
        現行工作模式,
        開啓反查輸入,
        開啓練習題選單,
        關閉輸入欄,
        開啓方案選單,
        開啓配列選單,
        ..
    } = 模式;
    let 作業機關輸出信號 {
        當前作業,
        佈置作業,
        目標作業內容,
        目標輸入碼片段,
        ..
    } = 作業;
    let 字幕機關輸出信號 { .. } = 字幕;
    let 連擊機關輸出信號 {
        實況字根碼,
        已錄入字根碼,
        逐鍵提示,
        連擊片段完成,
        ..
    } = 連擊;
    let 並擊機關輸出信號 {
        並擊狀態流,
        反查鍵位,
        反查所得並擊碼,
        實況並擊碼,
        並擊所得拼音,
        並擊完成,
        並擊成功,
        ..
    } = 並擊;

    let 是否顯示光標 = Signal::derive(move || matches!(指法(), 觸鍵方式::連擊));
    let 有無輸入碼 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => !實況字根碼.read().is_empty(),
        觸鍵方式::並擊 => !實況並擊碼.read().is_empty(),
    });
    let 顯示選項 = Signal::derive(move || {
        if 反查鍵位.read().is_some() {
            編碼欄顯示選項::顯示反查
        } else if 有無輸入碼() {
            編碼欄顯示選項::顯示實況
        } else {
            編碼欄顯示選項::無顯示
        }
    });
    let 輸入正確 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => 連擊片段完成(),
        觸鍵方式::並擊 => 並擊完成() && 並擊成功(),
    });
    let 點擊編碼欄動作 = move || {
        if 現行工作模式() == 工作模式::錄入 {
            if 當前作業.read().是否練習題() {
                開啓練習題選單();
            } else {
                開啓反查輸入();
            }
        }
    };
    let 編碼回顯區佈局 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => 回顯區佈局::單欄,
        觸鍵方式::並擊 => 回顯區佈局::左右對照,
    });
    let 回顯輸入碼 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => {
            let 輸入碼 = 已錄入字根碼();
            match 輸入碼.as_str() {
                "" | "␣" => 輸入碼,
                _ => 輸入碼 + "‸",
            }
        }
        觸鍵方式::並擊 => 反查所得並擊碼().unwrap_or_else(實況並擊碼),
    });
    let 回顯轉寫碼 = Signal::derive(move || match 指法() {
        觸鍵方式::連擊 => None,
        觸鍵方式::並擊 => {
            目標輸入碼片段()
                .and_then(|輸入碼| 輸入碼.轉寫碼原文)
                .or_else(並擊所得拼音)
                // 加尖括弧表示拉丁文轉寫
                .map(|轉寫| format!("⟨{轉寫}⟩"))
        }
    });
    let 反查碼 = Signal::derive(move || {
        目標作業內容
            .read()
            .as_ref()
            .flatten()
            .map(|作業| 作業.碼表.碼表原文().to_string())
    });
    let 反查碼變更動作 = move |反查碼: String| {
        佈置作業(作業::自訂(現行方案(), 反查碼));
    };
    let 當選題號 = Signal::derive(move || 當前作業.read().題號);
    let 選中題號動作 = move |題號| {
        佈置作業(作業::練習題(現行方案(), 題號));
        關閉輸入欄();
    };
    let 選中方案動作 = move |選中項| {
        選用方案(選中項);
        關閉輸入欄();
    };
    let 選中配列動作 = move |選中項| {
        選用配列(Some(選中項));
        關閉輸入欄();
    };
    let 方案配套練習題 = Signal::derive(move || 現行方案().配套練習題().unwrap_or(&[]));
    let 方案指定佈局 = Signal::derive(move || *方案定義.read().佈局);

    let 標註功能鍵 = |功能鍵| Signal::derive(move || 功能鍵);

    let 目標鍵位表示 = Signal::derive(move || match 指法() {
        觸鍵方式::並擊 => 反查鍵位(),
        觸鍵方式::連擊 => 逐鍵提示(),
    });

    let 並擊動態 = 並擊對標動態 {
        目標並擊: 目標鍵位表示,
        實況並擊: 並擊狀態流.into(),
    };

    let 開關狀態 = 功能鍵開關狀態 { 現行工作模式 };

    view! {
        <Rime字幕屏 是否顯示光標={是否顯示光標} 按進度顯示字幕={字幕.段落表示}/>
        <div class="echo-bar">
            <div title="重新錄入／選練習題">
                <Rime鍵圖 鍵={退出鍵.鍵碼} 標註法={標註功能鍵(退出鍵)} 着色法={開關狀態}/>
            </div>
            <div title="下一題">
                <Rime鍵圖 鍵={製表鍵.鍵碼} 標註法={標註功能鍵(製表鍵)} 着色法={並擊動態}/>
            </div>
            <Rime編碼欄
                顯示選項={顯示選項}
                輸入正確={輸入正確}
                點擊動作={點擊編碼欄動作}
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
                            反查碼變更={反查碼變更動作}
                        />
                    }.into_any(),
                    工作模式::選取練習題 => view! {
                        <Rime練習題選單
                            預設練習題={方案配套練習題}
                            當選題號={當選題號}
                            選中題號={選中題號動作}
                        />
                    }.into_any(),
                    工作模式::選擇輸入方案 => view! {
                        <Rime方案選單
                            現行方案={現行方案}
                            選中方案={選中方案動作}
                        />
                    }.into_any(),
                    工作模式::選擇配列 => view! {
                        <Rime配列選單
                            已選配列={實際配列}
                            選中配列={選中配列動作}
                        />
                    }.into_any(),
                }
            }
            </Rime編碼欄>
            <div title="輸入拼音反查鍵位">
                <Rime鍵圖 鍵={回車鍵.鍵碼} 標註法={標註功能鍵(回車鍵)} 着色法={開關狀態}/>
            </div>
            <div title="刪除／回退一字">
                <Rime鍵圖 鍵={退格鍵.鍵碼} 標註法={標註功能鍵(退格鍵)} 着色法={並擊動態}/>
            </div>
        </div>
        <Rime鍵盤圖 鍵盤佈局={方案指定佈局} 目標盤面={當選盤面} 配列={實際配列} 着色法={並擊動態}/>

        <Rime狀態欄
            現行方案={現行方案}
            已選配列={實際配列}
            點擊方案={move || 開啓方案選單()}
            點擊配列={move || 開啓配列選單()}
        />
    }
}
