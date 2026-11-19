use std::collections::BTreeSet;

use leptos::logging::log;
use leptos::prelude::*;

use crate::action::{動作, 動作得一結果, 動作給一參數, 未有};
use crate::definition::{判定鍵位, 碼表格式, 選擇鍵面, 鍵組};
use crate::gear::{
    assignment::作業機關輸出信號, layout::佈局機關輸出信號, theory::輸入方案機關輸出信號,
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
pub type 切盤動作 = impl 動作;

/// [連擊] 編碼回顯區的分段對照描述: 拆分已對前綴, 錯碼段與待輸霧化虛影.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct 連擊對照表示 {
    /// 是否採用分段對照視圖 (`碼表格式::連擊` 爲 `true`; `碼表格式::逐鍵` 爲 `false`)
    pub 分段對照: bool,
    /// 已正確匹配的字根前綴 (或自習/完成態下的完整已錄字根)
    pub 已對前綴: String,
    /// 與目標不符的錯碼後綴 (非空時即處於錯碼警示態)
    pub 錯碼段: String,
    /// 尚未錄入的目標字根序列 (用於渲染逐字霧化虛影)
    pub 待輸後綴: String,
    /// 是否顯示錄入插入符 `‸`
    pub 顯示光標: bool,
}

#[derive(Clone)]
pub struct 連擊機關輸出信號 {
    pub 連擊狀態變更: WriteSignal<連擊狀態>,
    // pub 連擊輸入碼: ReadSignal<Vec<String>>,
    pub 實況字根碼: Signal<String>,
    pub 已錄入字根碼: Memo<String>,
    pub 逐鍵提示: Signal<Option<鍵組>>,
    pub 餘碼提示: Signal<Option<鍵組>>,
    pub 同鍵連打數: Signal<usize>,
    pub 提示步進: Signal<usize>,
    pub 存在錯碼: Signal<bool>,
    pub 連擊對照: Signal<連擊對照表示>,
    pub 連擊比對成功: Memo<bool>,
    pub 連擊片段完成: Signal<bool>,
    pub 清空連擊輸入碼: 清空動作,
    pub 回退連擊輸入碼: 回退動作,
    pub 編輯連擊輸入碼: 擊鍵動作,
    pub 自動切換盤面: 切盤動作,
}

#[define_opaque(清空動作, 回退動作, 擊鍵動作, 切盤動作)]
pub fn 連擊機關(
    方案: &輸入方案機關輸出信號,
    作業: &作業機關輸出信號,
    佈局: &佈局機關輸出信號,
) -> 連擊機關輸出信號 {
    let 方案 = 方案.方案定義;
    let 目標碼表格式 = 作業.目標碼表格式;
    let 目標輸入碼片段 = 作業.目標輸入碼片段;
    let 作業進度 = 作業.作業進度;
    let 作業進度完成 = 作業.作業進度完成;
    let 當選盤面 = 佈局.當選盤面;
    let 選擇盤面 = 佈局.選擇盤面;

    let (連擊狀態流, 連擊狀態變更) = signal(連擊狀態::default());

    let 實況字根碼 = Signal::derive(move || {
        方案.read().寫成字根碼(&選擇鍵面 {
            盤面: 當選盤面(),
            鍵碼: 連擊狀態流.read().鍵碼,
        })
    });

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
        更新連擊輸入碼.update(|列表| {
            if !列表.is_empty() {
                列表.clear();
            }
        });
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

    let 編碼法 = move || 目標碼表格式.read().unwrap_or_else(|| 方案.read().編碼法);

    let 存在錯碼 = Signal::derive(move || {
        matches!(編碼法(), 碼表格式::連擊)
            && !作業進度完成()
            && 連擊進度() == 作業進度()
            && 反查所得字根碼.read().is_some()
            && 已錄入字根碼.read().chars().count() > 已正確錄入碼長()
    });

    let 逐鍵提示 = Signal::derive(move || {
        if 作業進度完成() {
            return None;
        }
        if 存在錯碼() {
            // 錯碼方案 A: 熄滅字母區下一鍵提示, 改爲點亮退格鍵引導糾錯
            return Some(鍵組(BTreeSet::from([KeyCode::Backspace])));
        }
        反查所得字根碼
            .read()
            .as_deref()
            .and_then(|字根碼| {
                let 碼長 = 已正確錄入碼長();
                字根碼.chars().nth(碼長)
            })
            .map(|字根碼| 方案.read().讀出鍵位(&字根碼.to_string(), &當選盤面()))
    });

    let 餘碼提示 = Signal::derive(move || {
        if !matches!(編碼法(), 碼表格式::連擊) || 作業進度完成() || 存在錯碼() {
            return None;
        }
        let 餘下字根: String = 反查所得字根碼
            .read()
            .as_deref()
            .map(|字根碼| 字根碼.chars().skip(已正確錄入碼長() + 1).collect())
            .unwrap_or_default();
        if 餘下字根.is_empty() {
            return None;
        }
        let mut 餘鍵 = 方案.read().讀出鍵位(&餘下字根, &當選盤面());
        if let Some(當前鍵) = 逐鍵提示() {
            for 鍵 in &當前鍵.0 {
                餘鍵.0.remove(鍵);
            }
        }
        餘鍵.有無鍵位().then_some(餘鍵)
    });

    let 同鍵連打數 = Signal::derive(move || {
        if !matches!(編碼法(), 碼表格式::連擊) || 作業進度完成() || 存在錯碼() {
            return 1;
        }
        反查所得字根碼
            .read()
            .as_deref()
            .map(|字根碼| {
                let mut 餘下 = 字根碼.chars().skip(已正確錄入碼長());
                let Some(首字根) = 餘下.next() else {
                    return 1;
                };
                1 + 餘下.take_while(|&後續| 後續 == 首字根).count()
            })
            .unwrap_or(1)
    });

    let 提示步進 = Signal::derive(move || 作業進度().wrapping_add(已正確錄入碼長()));

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

    let 連擊對照 = Signal::derive(move || {
        let 錄入 = 已錄入字根碼();
        if !matches!(編碼法(), 碼表格式::連擊) {
            let 顯示光標 = !matches!(錄入.as_str(), "" | "␣");
            return 連擊對照表示 {
                分段對照: false,
                已對前綴: 錄入,
                錯碼段: String::new(),
                待輸後綴: String::new(),
                顯示光標,
            };
        }

        if 連擊片段完成() {
            return 連擊對照表示 {
                分段對照: true,
                已對前綴: 錄入,
                錯碼段: String::new(),
                待輸後綴: String::new(),
                顯示光標: false,
            };
        }

        if 作業進度完成() {
            return 連擊對照表示 {
                分段對照: true,
                ..Default::default()
            };
        }

        if let Some(目標) = 反查所得字根碼.read().as_deref() {
            let 正確數 = 目標
                .chars()
                .zip(錄入.chars())
                .take_while(|(查得, 實錄)| 查得 == 實錄)
                .count();
            let 已對前綴: String = 錄入.chars().take(正確數).collect();
            let 錯碼段: String = 錄入.chars().skip(正確數).collect();
            let 待輸後綴: String = 目標.chars().skip(正確數).collect();
            let 顯示光標 = !錄入.is_empty();
            連擊對照表示 {
                分段對照: true,
                已對前綴,
                錯碼段,
                待輸後綴,
                顯示光標,
            }
        } else {
            let 顯示光標 = !錄入.is_empty();
            連擊對照表示 {
                分段對照: true,
                已對前綴: 錄入,
                錯碼段: String::new(),
                待輸後綴: String::new(),
                顯示光標,
            }
        }
    });

    let 位於碼段邊界 = move || {
        let 輸入碼 = 連擊輸入碼.read();
        輸入碼.last().is_some_and(|末位輸入碼| {
            let 方案 = 方案.read();
            方案.查分隔鍵(|鍵位| 鍵位.輸入碼 == 末位輸入碼).is_some()
                || 方案.查終止鍵(|鍵位| 鍵位.輸入碼 == 末位輸入碼).is_some()
        })
    };

    let 碼段起始鍵 =
        move |鍵面: &選擇鍵面| 方案.read().查起始鍵(|鍵位| 鍵面.包含鍵位(鍵位)).is_some();

    let 編輯連擊輸入碼 = move |鍵碼: KeyCode| {
        let 鍵面 = 選擇鍵面 {
            盤面: 當選盤面(),
            鍵碼,
        };
        let 字根碼 = 方案.read().寫成字根碼(&鍵面);
        log!("編輯連擊輸入碼 字根碼: {}", 字根碼);
        if 鍵碼 == KeyCode::Space
            && matches!(編碼法(), 碼表格式::連擊)
            && 作業進度() > 連擊進度()
            && (字根碼.is_empty()
                || !反查所得字根碼
                    .read()
                    .as_deref()
                    .is_some_and(|目標| 目標.starts_with(&字根碼)))
        {
            // 完成上一字後擊空格: 結束青色完成回顯態, 轉入下一字起點虛影預顯態
            清空連擊輸入碼();
            return;
        }
        if !字根碼.is_empty() {
            match 編碼法() {
                碼表格式::逐鍵 => {
                    let 自由輸入 = 目標輸入碼片段.read().is_none();
                    if 自由輸入 || 連擊比對成功() {
                        if 實錄分隔鍵().is_some() || 位於碼段邊界() {
                            清空連擊輸入碼();
                        }
                        更新連擊輸入碼.write().push(字根碼);
                    }
                }
                碼表格式::連擊 => {
                    if 作業進度() > 連擊進度() || 位於碼段邊界() || 碼段起始鍵(&鍵面)
                    {
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

    let 自動切換盤面 = move || {
        if let 碼表格式::連擊 = 編碼法() {
            let 輸入碼 = 已錄入字根碼();
            let 方案定義 = 方案.read();
            let 目標盤面 = 方案定義
                .動態切換
                .iter()
                .find_map(|規則| 規則.匹配輸入.is_match(&輸入碼).then_some(規則.目標盤面))
                .unwrap_or(方案定義.佈局.默認盤面);
            if 目標盤面 != 當選盤面() {
                log!("自動切換盤面啦: {:?}", 目標盤面);
                選擇盤面(目標盤面);
            }
        }
    };

    連擊機關輸出信號 {
        連擊狀態變更,
        // 連擊輸入碼,
        實況字根碼,
        已錄入字根碼,
        逐鍵提示,
        餘碼提示,
        同鍵連打數,
        提示步進,
        存在錯碼,
        連擊對照,
        連擊比對成功,
        連擊片段完成,
        清空連擊輸入碼,
        回退連擊輸入碼,
        編輯連擊輸入碼,
        自動切換盤面,
    }
}
