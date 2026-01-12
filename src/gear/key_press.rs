use leptos::prelude::*;

use crate::action::{動作, 動作得一結果, 動作給一參數, 未有};
use crate::definition::{碼表格式, 鍵組};
use crate::gear::{
    assignment::{作業機關輸出信號, 碼表定義},
    theory::輸入方案機關輸出信號,
};
use crate::key_code::KeyCode;
use crate::spelling_algebra::施展拼寫運算;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct 連擊狀態 {
    pub 鍵碼: KeyCode,
    pub 連擊次數: usize,
}

impl Default for 連擊狀態 {
    fn default() -> Self {
        Self {
            鍵碼: KeyCode::No,
            連擊次數: 0,
        }
    }
}

impl 連擊狀態 {
    pub fn 擊發(&mut self, 鍵碼: KeyCode) {
        if 鍵碼 == self.鍵碼 {
            self.連擊次數 += 1;
        } else {
            self.鍵碼 = 鍵碼;
            self.連擊次數 = 1;
        }
    }
}

pub type 清空動作 = impl 動作;
pub type 回退動作 = impl 動作得一結果;
pub type 擊鍵動作 = impl 動作給一參數<KeyCode>;

#[derive(Clone)]
pub struct 連擊機關輸出信號 {
    pub 連擊狀態變更: WriteSignal<連擊狀態>,
    pub 連擊輸入碼: ReadSignal<Vec<String>>,
    pub 實況字根碼: Signal<String>,
    pub 逐鍵提示: Signal<Option<鍵組>>,
    pub 連擊比對成功: Memo<bool>,
    pub 連擊片段完成: Signal<bool>,
    pub 清空連擊輸入碼: 清空動作,
    pub 回退連擊輸入碼: 回退動作,
    pub 編輯連擊輸入碼: 擊鍵動作,
}

#[define_opaque(清空動作, 回退動作, 擊鍵動作)]
pub fn 連擊機關(
    方案: &輸入方案機關輸出信號,
    作業: &作業機關輸出信號,
) -> 連擊機關輸出信號 {
    let 方案 = 方案.方案定義;
    let 當前作業 = 作業.當前作業;
    let 目標輸入碼片段 = 作業.目標輸入碼片段;
    let 作業進度 = 作業.作業進度;
    let 作業進度完成 = 作業.作業進度完成;

    let (連擊狀態流, 連擊狀態變更) = signal(連擊狀態::default());

    let 實況字根碼 = Signal::derive(move || 方案.read().寫成字根碼(連擊狀態流.read().鍵碼));

    let 反查所得字根碼 = Memo::new(move |_| {
        目標輸入碼片段
            .read()
            .as_ref()
            .and_then(|對照碼| 對照碼.反查字根碼(&方案.read().轉寫法))
    });

    let 實錄分隔鍵 = move || {
        let 實錄鍵碼 = 連擊狀態流.read().鍵碼;
        方案.read().查分隔鍵(move |鍵位| 鍵位.鍵碼 == 實錄鍵碼)
    };

    let (連擊輸入碼, 更新連擊輸入碼) = signal(Vec::<String>::new());

    let (連擊進度, 更新連擊進度) = signal(0);

    let 清空連擊輸入碼 = move || {
        更新連擊輸入碼.write().clear();
        let 校正進度 = 作業進度();
        if 連擊進度() != 校正進度 {
            更新連擊進度(校正進度);
        }
    };

    let 回退連擊輸入碼 = move || {
        if 作業進度() > 連擊進度() {
            // 完成狀態下回退一字
            return Err(未有());
        }
        更新連擊輸入碼.write().pop().map(|_| ()).ok_or(未有())
    };

    let 已錄入字根碼 = Memo::new(move |_| {
        let 字根碼 = 連擊輸入碼.read().join("");
        施展拼寫運算(&字根碼, 方案.read().轉寫法.輸入碼表示).unwrap_or(字根碼)
    });

    let 已正確錄入碼長 = move || {
        if 連擊進度() == 作業進度() {
            反查所得字根碼.read().as_deref().map_or(0, |查得| {
                let 錄入 = 已錄入字根碼();
                查得
                    .chars()
                    .zip(錄入.chars())
                    .take_while(|(查得, 錄入)| 查得 == 錄入)
                    .count()
            })
        } else {
            0
        }
    };

    let 逐鍵提示 = Signal::derive(move || {
        反查所得字根碼
            .read()
            .as_deref()
            .and_then(|字根碼| {
                let 碼長 = 已正確錄入碼長();
                字根碼.chars().nth(碼長)
            })
            .map(|字根碼| 方案.read().讀出鍵位(&字根碼.to_string()))
    });

    let 編碼法 = move || {
        當前作業
            .read()
            .目標輸入碼()
            .as_ref()
            .and_then(碼表定義::碼表格式)
            .unwrap_or_else(|| 方案.read().編碼法)
    };

    let 連擊比對成功 = Memo::new(move |_| {
        let 字根碼 = 實錄分隔鍵().map_or_else(
            || match 編碼法() {
                碼表格式::逐鍵 => 實況字根碼(),
                碼表格式::連擊 => 已錄入字根碼(),
                _ => "".to_string(),
            },
            |分隔鍵| 分隔鍵.輸入碼.to_string(),
        );
        反查所得字根碼
            .read()
            .as_deref()
            .is_some_and(|查得| 查得 == 字根碼)
    });

    let 完成一詞 = move || {
        作業進度完成()
            || 目標輸入碼片段
                .read()
                .as_ref()
                .and_then(|對照碼| 對照碼.轉寫碼原文.as_deref())
                .is_some_and(|轉寫碼| 轉寫碼 == " ")
    };

    let 連擊片段完成 = Signal::derive(move || match 編碼法() {
        碼表格式::逐鍵 => 完成一詞(),
        碼表格式::連擊 => 作業進度() > 連擊進度(),
        _ => false,
    });

    let 位於碼段邊界 = move || {
        let 輸入碼 = 連擊輸入碼.read();
        輸入碼.last().is_some_and(|末位輸入碼| {
            let 方案 = 方案.read();
            方案.查分隔鍵(|鍵位| 鍵位.輸入碼 == 末位輸入碼).is_some()
                || 方案.查終止鍵(|鍵位| 鍵位.輸入碼 == 末位輸入碼).is_some()
        })
    };

    let 編輯連擊輸入碼 = move |鍵碼: KeyCode| {
        let 自由輸入 = 目標輸入碼片段.read().is_none();
        let 字根碼 = 方案.read().寫成字根碼(鍵碼);
        if !字根碼.is_empty() {
            match 編碼法() {
                碼表格式::逐鍵 => {
                    if 自由輸入 || 連擊比對成功() {
                        if 實錄分隔鍵().is_some() || 位於碼段邊界() {
                            清空連擊輸入碼();
                        }
                        更新連擊輸入碼.write().push(字根碼);
                    }
                }
                碼表格式::連擊 => {
                    if 作業進度() > 連擊進度() || 位於碼段邊界() {
                        清空連擊輸入碼();
                    }
                    更新連擊輸入碼.write().push(字根碼);
                    // 若連擊比對成功, 作業將隨即推進至下一片段
                    // 先不要清空碼段, 顯示連擊片段完成狀態, 待下一擊再清空
                }
                _ => (),
            }
        }
    };

    連擊機關輸出信號 {
        連擊狀態變更,
        連擊輸入碼,
        實況字根碼,
        逐鍵提示,
        連擊比對成功,
        連擊片段完成,
        清空連擊輸入碼,
        回退連擊輸入碼,
        編輯連擊輸入碼,
    }
}
