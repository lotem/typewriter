use leptos::logging::log;
use leptos::prelude::*;

use crate::action::動作;
use crate::definition::觸鍵方式;
use crate::gear::{
    assignment::{作業, 作業機關, 作業機關輸出信號, 步進法},
    caption::{字幕機關, 字幕機關輸出信號, 字幕段落},
    chord::{並擊機關, 並擊機關輸出信號},
    input::{焦點事件處理機關, 觸鍵消息, 輸入事件處理機關},
    key_press::{連擊機關, 連擊機關輸出信號},
    layout::{配列機關, 配列機關輸出信號},
    mode::{工作模式, 工作模式機關, 工作模式機關輸出信號},
    theory::{輸入方案機關, 輸入方案機關輸出信號},
};
use crate::key_code::KeyCode;

#[derive(Clone)]
pub struct 微觀引擎輸出信號 {
    pub 方案: 輸入方案機關輸出信號,
    pub 模式: 工作模式機關輸出信號,
    pub 配列: 配列機關輸出信號,
    pub 作業: 作業機關輸出信號,
    pub 字幕: 字幕機關輸出信號,
    pub 連擊: 連擊機關輸出信號,
    pub 並擊: 並擊機關輸出信號,
}

pub fn 微觀引擎() -> 微觀引擎輸出信號 {
    let 方案 = 輸入方案機關();
    let 配列 = 配列機關(&方案);
    let 作業 = 作業機關(&方案);
    let 字幕 = 字幕機關(&方案, &作業);
    let 連擊 = 連擊機關(&方案, &作業);
    let 並擊 = 並擊機關(&方案, &作業);

    let 輸入方案機關輸出信號 {
        現行方案, 指法,
    ..
    } = 方案;
    let 作業機關輸出信號 {
        佈置作業,
        作業進度,
        重置作業進度,
        作業進度完成,
        作業推進,
        作業回退,
        有無作業,
        ..
    } = 作業;
    let 字幕機關輸出信號 {
        分段字幕,
        當前段落,
        前序段落,
        ..
    } = 字幕;
    let 連擊機關輸出信號 {
        連擊狀態變更,
        連擊比對成功,
        清空連擊輸入碼,
        回退連擊輸入碼,
        編輯連擊輸入碼,
        ..
    } = 連擊;
    let 並擊機關輸出信號 {
        並擊狀態變更,
        重置並擊狀態,
        並擊完成,
        並擊成功,
        ..
    } = 並擊;

    let 輸入 = 輸入動作機關(&方案, &連擊, &並擊);

    let 模式 = 工作模式機關(&方案, &作業, &輸入);

    let 輸入動作 { 重置輸入狀態 } = 輸入;

    let 工作模式機關輸出信號 {
        現行工作模式,
        開啓反查輸入,
        開啓練習題選單,
        開啓方案選單,
        開啓配列選單,
        關閉輸入欄,
        ..
    } = 模式;

    let _ = Effect::watch(
        現行工作模式,
        move |新, 舊, _| {
            log!("工作模式: {:?} -> {:?}", 舊, 新);
        },
        false,
    );

    焦點事件處理機關(重置並擊狀態);

    let 處理功能鍵 = move |觸鍵消息 { 鍵碼, 檔位 }| match 鍵碼 {
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
            match 現行工作模式() {
                工作模式::錄入 => {
                    let 跳轉結果 = if 檔位.shift {
                        let 目標 = 前序段落()
                            .or_else(|| 分段字幕.with(|衆段落| 衆段落.last().cloned()))
                            .map(|字幕段落(起, _, _)| 起);
                        作業回退(步進法 {
                            目標, 迴轉: true
                        })
                    } else {
                        let 目標 = 當前段落().map(|字幕段落(_, 止, _)| 止);
                        作業推進(步進法 {
                            目標, 迴轉: true
                        })
                    };
                    if 跳轉結果.is_ok() {
                        重置輸入狀態();
                    }
                }
                工作模式::選取練習題 => {
                    開啓方案選單();
                }
                工作模式::選擇輸入方案 => {
                    開啓配列選單();
                }
                工作模式::選擇配列 => {
                    開啓練習題選單();
                }
                _ => {
                    關閉輸入欄();
                }
            }
            true
        }
        KeyCode::Backspace => {
            if 現行工作模式() == 工作模式::錄入 {
                match 指法() {
                    觸鍵方式::連擊 => {
                        if 有無作業() {
                            let _不看結果 = 作業回退(步進法::default());
                        }
                        回退連擊輸入碼();
                    }
                    觸鍵方式::並擊 => {
                        if 並擊完成() || 作業回退(步進法::default()).is_ok() {
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
        let 分段落則迴轉 = 步進法 {
            目標: None,
            迴轉: 分段字幕.read().len() > 1,
        };
        擊中目標() && 作業推進(分段落則迴轉).is_ok()
    };
    let 另起一段 = move || {
        當前段落
            .read()
            .as_ref()
            .is_some_and(|字幕段落(段落起始, _, _)| 作業進度.read() == *段落起始)
    };
    let 既然落鍵 = move |鍵碼| {
        // 繼續擊鍵時消除已完成的反查作業
        if 作業進度完成() {
            佈置作業(作業::自習(現行方案()));
        }
        if 現行工作模式() == 工作模式::錄入 {
            並擊狀態變更.write().落鍵(鍵碼);
            if 指法() == 觸鍵方式::連擊 {
                連擊狀態變更.write().擊發(鍵碼);
                編輯連擊輸入碼(鍵碼);
                if 批閱作業() && 另起一段() {
                    清空連擊輸入碼();
                }
            }
        }
    };
    let 既然抬鍵 = move |鍵碼| {
        if 現行工作模式() == 工作模式::錄入 {
            並擊狀態變更.write().抬鍵(鍵碼);
        }
        match 指法() {
            觸鍵方式::連擊 => {
                // 顯示並擊動態, 抬鍵後清除並擊結果
                if 並擊完成() {
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

    微觀引擎輸出信號 {
        方案,
        模式,
        配列,
        作業,
        字幕,
        連擊,
        並擊,
    }
}

pub type 輸入重置動作 = impl 動作;

#[derive(Clone)]
pub struct 輸入動作 {
    pub 重置輸入狀態: 輸入重置動作,
}

#[define_opaque(輸入重置動作)]
fn 輸入動作機關(
    方案: &輸入方案機關輸出信號,
    連擊: &連擊機關輸出信號,
    並擊: &並擊機關輸出信號,
) -> 輸入動作 {
    let 指法 = 方案.指法;
    let 清空連擊輸入碼 = 連擊.清空連擊輸入碼;
    let 重置並擊狀態 = 並擊.重置並擊狀態;

    let 重置輸入狀態 = move || match 指法() {
        觸鍵方式::連擊 => 清空連擊輸入碼(),
        觸鍵方式::並擊 => 重置並擊狀態(),
    };

    輸入動作 { 重置輸入狀態 }
}
