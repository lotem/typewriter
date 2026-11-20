use std::collections::BTreeSet;

use leptos::logging::log;
use leptos::prelude::*;

use crate::action::{動作, 動作得一結果, 動作給一參數, 未有};
use crate::definition::{判定鍵位, 碼表格式, 輸入方案定義, 選擇鍵面, 鍵組};
use crate::gear::{
    assignment::{作業機關輸出信號, 對照輸入碼},
    layout::{佈局機關輸出信號, 盤面選擇碼},
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

/// 顯式連擊緩衝三態機: 取代舊有 `作業進度 > 連擊進度` 雙計數器隱式差值推斷.
#[derive(Clone, Debug, Default, PartialEq)]
enum 連擊緩衝 {
    /// 空緩衝起點態: 若有目標碼表則直接展示待輸霧化虛影
    #[default]
    待命,
    /// 正在拼打當前字詞: 顯示插入符光標 `‸` (未達碼段封口邊界時), 並參與目標比對與錯碼檢測
    錄入中(Vec<String>),
    /// 剛完成上一字詞並保留青色成果回顯: 隱藏光標, 遇收結鍵轉 `待命`, 遇新字根鍵自動覆寫爲新段 `錄入中`
    完成駐留(Vec<String>),
}

impl 連擊緩衝 {
    fn 是否錄入中(&self) -> bool {
        matches!(self, Self::錄入中(_))
    }

    fn 是否完成駐留(&self) -> bool {
        matches!(self, Self::完成駐留(_))
    }

    fn 末位輸入碼(&self) -> Option<&str> {
        match self {
            Self::待命 => None,
            Self::錄入中(碼組) | Self::完成駐留(碼組) => 碼組.last().map(String::as_str),
        }
    }

    fn 拼接原文(&self) -> String {
        match self {
            Self::待命 => String::new(),
            Self::錄入中(碼組) | Self::完成駐留(碼組) => 碼組.join(""),
        }
    }
}

pub type 清空動作 = impl 動作;
pub type 完成動作 = impl 動作;
pub type 回退動作 = impl 動作得一結果;
pub type 擊鍵動作 = impl 動作給一參數<KeyCode>;
pub type 切盤動作 = impl 動作;

/// 單欄編碼回顯區的統一對照描述 (`逐鍵` 與 `連擊` 共用單一渲染通道).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct 連擊對照表示 {
    /// 已正確匹配的字根前綴 (或自習/完成駐留態下的完整已錄字根)
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
    pub 連擊狀態流: ReadSignal<連擊狀態>,
    pub 連擊狀態變更: WriteSignal<連擊狀態>,
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
    pub 完成連擊碼段: 完成動作,
    pub 回退連擊輸入碼: 回退動作,
    pub 編輯連擊輸入碼: 擊鍵動作,
    pub 自動切換盤面: 切盤動作,
}

pub fn 連擊機關(
    方案: &輸入方案機關輸出信號,
    作業: &作業機關輸出信號,
    佈局: &佈局機關輸出信號,
) -> 連擊機關輸出信號 {
    構建連擊機關(
        方案.方案定義,
        作業.目標碼表格式,
        作業.目標輸入碼片段,
        作業.作業進度,
        作業.作業進度完成,
        佈局.當選盤面,
        佈局.選擇盤面,
    )
}

#[define_opaque(清空動作, 完成動作, 回退動作, 擊鍵動作, 切盤動作)]
fn 構建連擊機關(
    方案: Signal<輸入方案定義<'static>>,
    目標碼表格式: Signal<Option<碼表格式>>,
    目標輸入碼片段: Signal<Option<對照輸入碼>>,
    作業進度: ReadSignal<usize>,
    作業進度完成: Signal<bool>,
    當選盤面: ReadSignal<盤面選擇碼>,
    選擇盤面: WriteSignal<盤面選擇碼>,
) -> 連擊機關輸出信號 {
    let (連擊狀態流, 連擊狀態變更) = signal(連擊狀態::default());
    let (連擊緩衝流, 更新連擊緩衝) = signal(連擊緩衝::default());

    let 編碼法 = move || 目標碼表格式.read().unwrap_or_else(|| 方案.read().編碼法);

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

    let 清空連擊輸入碼 = move || {
        更新連擊緩衝.update(|緩衝| {
            if *緩衝 != 連擊緩衝::待命 {
                *緩衝 = 連擊緩衝::待命;
            }
        });
    };

    let 完成連擊碼段 = move || {
        let 應封存駐留 = match 編碼法() {
            碼表格式::連擊 => true,
            碼表格式::逐鍵 => {
                let 方案定義 = 方案.read();
                let 本段是分隔 = 連擊緩衝流.with(|緩衝| {
                    緩衝
                        .末位輸入碼()
                        .is_some_and(|末碼| 方案定義.是否分隔碼(末碼))
                });
                let 下段是分隔 = 反查所得字根碼
                    .read()
                    .as_deref()
                    .is_some_and(|下碼| 方案定義.是否分隔碼(下碼));
                作業進度完成() || 本段是分隔 || 下段是分隔
            }
            _ => false,
        };
        if 應封存駐留 {
            更新連擊緩衝.update(|緩衝| {
                if let 連擊緩衝::錄入中(碼組) = 緩衝 {
                    *緩衝 = 連擊緩衝::完成駐留(std::mem::take(碼組));
                }
            });
        }
    };

    let 回退連擊輸入碼 = move || {
        let mut 已回退 = false;
        更新連擊緩衝.update(|緩衝| {
            if let 連擊緩衝::錄入中(碼組) = 緩衝 {
                if 碼組.pop().is_some() {
                    已回退 = true;
                    if 碼組.is_empty() {
                        *緩衝 = 連擊緩衝::待命;
                    }
                }
            }
        });
        已回退.then_some(()).ok_or(未有())
    };

    let 已錄入字根碼 = Memo::new(move |_| {
        let 字根碼 = 連擊緩衝流.read().拼接原文();
        if 字根碼.is_empty() {
            字根碼
        } else {
            施展拼寫運算(&字根碼, 方案.read().轉寫法.輸入碼表示).unwrap_or(字根碼)
        }
    });

    let 位於碼段邊界 = move || {
        連擊緩衝流.with(|緩衝| {
            緩衝.末位輸入碼().is_some_and(|末碼| {
                let 方案定義 = 方案.read();
                方案定義.是否分隔碼(末碼) || 方案定義.是否終止碼(末碼)
            })
        })
    };

    let 已正確錄入碼長 = move || {
        if !連擊緩衝流.read().是否錄入中() {
            return 0;
        }
        反查所得字根碼.read().as_deref().map_or(0, |查得| {
            let 錄入 = 已錄入字根碼();
            查得
                .chars()
                .zip(錄入.chars())
                .take_while(|(查得字, 錄入字)| 查得字 == 錄入字)
                .count()
        })
    };

    let 存在錯碼 = Signal::derive(move || {
        matches!(編碼法(), 碼表格式::連擊)
            && !作業進度完成()
            && 連擊緩衝流.read().是否錄入中()
            && 反查所得字根碼.read().is_some()
            && 已錄入字根碼.read().chars().count() > 已正確錄入碼長()
    });

    let 逐鍵提示 = Signal::derive(move || {
        if 作業進度完成() {
            return None;
        }
        if 存在錯碼() {
            return Some(鍵組(BTreeSet::from([KeyCode::Backspace])));
        }
        反查所得字根碼
            .read()
            .as_deref()
            .and_then(|字根碼| 字根碼.chars().nth(已正確錄入碼長()))
            .map(|字根| 方案.read().讀出鍵位(&字根.to_string(), &當選盤面()))
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
        if !連擊緩衝流.read().是否錄入中() {
            return false;
        }
        let 待比對碼 = match 編碼法() {
            碼表格式::逐鍵 => 實況字根碼(),
            碼表格式::連擊 => 已錄入字根碼(),
            _ => return false,
        };
        反查所得字根碼
            .read()
            .as_deref()
            .is_some_and(|查得| 查得 == 待比對碼)
    });

    let 連擊片段完成 = Signal::derive(move || 連擊緩衝流.read().是否完成駐留());

    let 連擊對照 = Signal::derive(move || {
        let 緩衝 = 連擊緩衝流.read();
        let 錄入 = 已錄入字根碼();

        match &*緩衝 {
            連擊緩衝::完成駐留(_) => 連擊對照表示 {
                已對前綴: 錄入,
                錯碼段: String::new(),
                待輸後綴: String::new(),
                顯示光標: false,
            },
            _ if !matches!(編碼法(), 碼表格式::連擊) => 連擊對照表示 {
                已對前綴: 錄入,
                錯碼段: String::new(),
                待輸後綴: String::new(),
                顯示光標: 緩衝.是否錄入中() && !位於碼段邊界(),
            },
            _ if 作業進度完成() => 連擊對照表示::default(),
            _ => {
                let 顯示光標 = 緩衝.是否錄入中() && !位於碼段邊界();
                if let Some(目標) = 反查所得字根碼.read().as_deref() {
                    let 正確數 = 目標
                        .chars()
                        .zip(錄入.chars())
                        .take_while(|(查得, 實錄)| 查得 == 實錄)
                        .count();
                    連擊對照表示 {
                        已對前綴: 錄入.chars().take(正確數).collect(),
                        錯碼段: 錄入.chars().skip(正確數).collect(),
                        待輸後綴: 目標.chars().skip(正確數).collect(),
                        顯示光標,
                    }
                } else {
                    連擊對照表示 {
                        已對前綴: 錄入,
                        錯碼段: String::new(),
                        待輸後綴: String::new(),
                        顯示光標,
                    }
                }
            }
        }
    });

    let 編輯連擊輸入碼 = move |鍵碼: KeyCode| {
        let 鍵面 = 選擇鍵面 {
            盤面: 當選盤面(),
            鍵碼,
        };
        let (字根碼, 是分隔鍵, 是起始鍵, 是終止鍵) = {
            let 方案定義 = 方案.read();
            (
                方案定義.寫成字根碼(&鍵面),
                方案定義.是否分隔鍵(&鍵面),
                方案定義.是否起始鍵(&鍵面),
                方案定義.是否終止鍵(&鍵面),
            )
        };
        log!("編輯連擊輸入碼 字根碼: {}", 字根碼);

        let 自由輸入 = 目標輸入碼片段.read().is_none();
        let 處於駐留態 = 連擊緩衝流.read().是否完成駐留();
        let 非目標首碼 = !反查所得字根碼
            .read()
            .as_deref()
            .is_some_and(|目標| !字根碼.is_empty() && 目標.starts_with(&字根碼));

        // 邊界收結規則 (統一由 `邊界判定規則` 驅動):
        // 1. 無字根分隔鍵 (如 [倉頡] / [動態能力注音] 斷字空格): 在完成駐留態或自習態下收結當前碼段;
        // 2. 兼作終止鍵的空格 (如 [注音] 第一聲 `ˉ`, 按定義只出現在碼段末尾): 在完成駐留態下收結前字並顯露後字.
        if (字根碼.is_empty() && 是分隔鍵 && (處於駐留態 || 自由輸入))
            || (處於駐留態 && 鍵碼 == KeyCode::Space && 是終止鍵 && 非目標首碼)
        {
            清空連擊輸入碼();
            return;
        }

        if 字根碼.is_empty() {
            return;
        }

        match 編碼法() {
            碼表格式::逐鍵 => {
                let 命中逐鍵目標 = 反查所得字根碼
                    .read()
                    .as_deref()
                    .is_some_and(|目標| 目標 == 字根碼);
                if 自由輸入 || 命中逐鍵目標 {
                    let 另起碼段 = 處於駐留態 || 是分隔鍵 || 位於碼段邊界();
                    更新連擊緩衝.update(|緩衝| match 緩衝 {
                        連擊緩衝::錄入中(碼組) if !另起碼段 => 碼組.push(字根碼),
                        _ => *緩衝 = 連擊緩衝::錄入中(vec![字根碼]),
                    });
                }
            }
            碼表格式::連擊 => {
                let 另起碼段 = 處於駐留態 || (位於碼段邊界() && !存在錯碼()) || 是起始鍵;
                更新連擊緩衝.update(|緩衝| match 緩衝 {
                    連擊緩衝::錄入中(碼組) if !另起碼段 => 碼組.push(字根碼),
                    _ => *緩衝 = 連擊緩衝::錄入中(vec![字根碼]),
                });
            }
            _ => (),
        }
    };

    let 自動切換盤面 = move || {
        if let 碼表格式::連擊 = 編碼法() {
            let 方案定義 = 方案.read();
            let 目標盤面 = if !連擊緩衝流.read().是否錄入中() || 位於碼段邊界() {
                方案定義.佈局.默認盤面
            } else {
                let 輸入碼 = 已錄入字根碼();
                方案定義
                    .動態切換
                    .iter()
                    .find_map(|規則| 規則.匹配輸入.is_match(&輸入碼).then_some(規則.目標盤面))
                    .unwrap_or(方案定義.佈局.默認盤面)
            };
            if 目標盤面 != 當選盤面() {
                log!("自動切換盤面啦: {:?}", 目標盤面);
                選擇盤面(目標盤面);
            }
        }
    };

    連擊機關輸出信號 {
        連擊狀態流,
        連擊狀態變更,
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
        完成連擊碼段,
        回退連擊輸入碼,
        編輯連擊輸入碼,
        自動切換盤面,
    }
}

#[cfg(test)]
mod 測試 {
    use super::*;
    use crate::gear::theory::輸入方案環境;
    use crate::theory::{
        alphabet::拉丁字母輸入方案, cangjie::倉頡輸入方案, detenele::動態能力注音輸入方案,
        zhuyin::注音輸入方案,
    };

    fn 測試環境() -> 輸入方案環境 {
        let 已選配列 = Memo::new(|_| None);
        輸入方案環境 { 已選配列 }
    }

    struct 連擊測試臺 {
        機關: 連擊機關輸出信號,
        進度寫: WriteSignal<usize>,
        總長: usize,
    }

    impl 連擊測試臺 {
        fn 建立(
            方案定義值: 輸入方案定義<'static>,
            格式覆寫: Option<碼表格式>,
            目標序列: Vec<對照輸入碼>,
        ) -> Self {
            let 總長 = 目標序列.len();
            let (方案源, _) = signal(方案定義值);
            let (格式源, _) = signal(格式覆寫);
            let (序列源, _) = signal(目標序列);
            let (進度讀, 進度寫) = signal(0usize);
            let 進度完成 = Signal::derive(move || 總長 > 0 && 進度讀.get() >= 總長);
            let 當前片段 = Signal::derive(move || {
                序列源.with(|列表| {
                    if 列表.is_empty() {
                        None
                    } else {
                        列表.get(進度讀.get().min(列表.len() - 1)).cloned()
                    }
                })
            });
            let (當選盤面, 選擇盤面) = signal(方案定義值.佈局.默認盤面);

            let 機關 = 構建連擊機關(
                方案源.into(),
                格式源.into(),
                當前片段,
                進度讀,
                進度完成,
                當選盤面,
                選擇盤面,
            );
            Self {
                機關, 進度寫, 總長
            }
        }

        /// 模擬 `engine.rs` 中單次落鍵與批閱流程 (單段落不迴轉).
        fn 擊鍵(&self, 鍵碼: KeyCode) {
            self.機關.連擊狀態變更.write().擊發(鍵碼);
            (self.機關.編輯連擊輸入碼)(鍵碼);
            if self.總長 > 0 && (self.機關.連擊比對成功)() {
                self.進度寫.update(|進度| *進度 += 1);
                (self.機關.完成連擊碼段)();
            }
            (self.機關.自動切換盤面)();
        }
    }

    fn 轉寫目標(文本: &str) -> 對照輸入碼 {
        對照輸入碼 {
            字根碼原文: None,
            轉寫碼原文: Some(文本.to_string()),
        }
    }

    fn 字根目標(文本: &str) -> 對照輸入碼 {
        對照輸入碼 {
            字根碼原文: Some(文本.to_string()),
            轉寫碼原文: None,
        }
    }

    #[test]
    fn 測試倉頡完成駐留與空格斷字及重字防誤進() {
        let 宿主 = Owner::new();
        宿主.with(|| {
            // 1. 練習態: 連續兩個相同字碼 "人" -> "人"
            let 臺 = 連擊測試臺::建立(
                倉頡輸入方案(測試環境()),
                None,
                vec![轉寫目標("人"), 轉寫目標("人")],
            );
            assert_eq!(臺.機關.連擊對照.get().待輸後綴, "人");

            // 擊 O ("人") 完成第 1 字, 進入完成駐留態
            臺.擊鍵(KeyCode::O);
            assert!(臺.機關.連擊片段完成.get());
            assert_eq!(臺.機關.連擊對照.get().已對前綴, "人");
            assert!(!臺.機關.連擊對照.get().顯示光標);

            // 在重字駐留態下誤觸未映射鍵 Z, 絕不可誤判第 2 個 "人" 比對成功
            臺.擊鍵(KeyCode::Z);
            assert!(臺.機關.連擊片段完成.get());

            // 擊空格 (分隔鍵) 收結前字駐留態, 顯露第 2 個 "人" 的待輸虛影
            臺.擊鍵(KeyCode::Space);
            assert!(!臺.機關.連擊片段完成.get());
            assert_eq!(臺.機關.連擊對照.get().已對前綴, "");
            assert_eq!(臺.機關.連擊對照.get().待輸後綴, "人");

            // 2. 自習態: 無目標序列時按空格可斷字清空
            let 自習臺 = 連擊測試臺::建立(倉頡輸入方案(測試環境()), None, vec![]);
            自習臺.擊鍵(KeyCode::D);
            自習臺.擊鍵(KeyCode::V);
            assert_eq!(自習臺.機關.已錄入字根碼.get(), "木女");
            assert!(自習臺.機關.連擊對照.get().顯示光標);
            自習臺.擊鍵(KeyCode::Space);
            assert_eq!(自習臺.機關.已錄入字根碼.get(), "");
            assert!(!自習臺.機關.連擊對照.get().顯示光標);
        });
    }

    #[test]
    fn 測試注音終止鍵空格兩用() {
        let 宿主 = Owner::new();
        宿主.with(|| {
            let 臺 = 連擊測試臺::建立(
                注音輸入方案(測試環境()),
                None,
                vec![轉寫目標("ㄓㄨㄥˉ"), 轉寫目標("ㄉㄧˋ")],
            );

            // 第一用: 拼字途中空格作陰平調號 `ˉ`
            臺.擊鍵(KeyCode::Kc5);
            臺.擊鍵(KeyCode::J);
            臺.擊鍵(KeyCode::Slash);
            assert!(!臺.機關.連擊片段完成.get());
            臺.擊鍵(KeyCode::Space);
            assert!(臺.機關.連擊片段完成.get());
            assert_eq!(臺.機關.連擊對照.get().已對前綴, "ㄓㄨㄥˉ");

            // 第二用: 完成駐留態下擊空格 (`ˉ` 屬終止鍵, 非下一字首碼), 收結前字而不報錯
            臺.擊鍵(KeyCode::Space);
            assert!(!臺.機關.連擊片段完成.get());
            assert!(!臺.機關.存在錯碼.get());
            assert_eq!(臺.機關.連擊對照.get().待輸後綴, "ㄉㄧˋ");

            // 對照驗證: 在待命起點誤觸二聲鍵 Kc6 (`ˊ`) 則正常標記錯碼
            臺.擊鍵(KeyCode::Kc6);
            assert!(臺.機關.存在錯碼.get());
            assert_eq!(臺.機關.連擊對照.get().錯碼段, "ˊ");
        });
    }

    #[test]
    fn 測試拉丁字母逐鍵詞尾駐留與空格步進() {
        let 宿主 = Owner::new();
        宿主.with(|| {
            let 臺 = 連擊測試臺::建立(
                拉丁字母輸入方案(測試環境()),
                Some(碼表格式::逐鍵),
                vec![轉寫目標("h"), 轉寫目標("e"), 轉寫目標(" "), 轉寫目標("w")],
            );

            臺.擊鍵(KeyCode::H);
            assert!(!臺.機關.連擊片段完成.get());
            assert_eq!(臺.機關.連擊對照.get().已對前綴, "H");
            assert!(臺.機關.連擊對照.get().顯示光標);

            // 打完詞尾 'e', 下一目標爲分隔碼 "␣", 自動進入完成駐留態 (青色無光標)
            臺.擊鍵(KeyCode::E);
            assert!(臺.機關.連擊片段完成.get());
            assert_eq!(臺.機關.連擊對照.get().已對前綴, "HE");
            assert!(!臺.機關.連擊對照.get().顯示光標);

            // 擊空格命中分隔碼 "␣", 覆寫駐留爲 "␣" (無光標) 並推進至 'w'
            臺.擊鍵(KeyCode::Space);
            assert!(臺.機關.連擊片段完成.get());
            assert_eq!(臺.機關.連擊對照.get().已對前綴, "␣");
            assert!(!臺.機關.連擊對照.get().顯示光標);

            // 擊 'w' 自動清除 "␣", 進入新詞錄入態 "W‸"
            臺.擊鍵(KeyCode::W);
            // 最末字符完成, 進入全文完成駐留態
            assert!(臺.機關.連擊片段完成.get());
            assert_eq!(臺.機關.連擊對照.get().已對前綴, "W");
            assert!(!臺.機關.連擊對照.get().顯示光標);
        });
    }

    #[test]
    fn 測試動態能力注音切盤與空格收結() {
        let 宿主 = Owner::new();
        宿主.with(|| {
            let 方案值 = 動態能力注音輸入方案(測試環境());
            let 默認聲母盤 = 方案值.佈局.默認盤面;
            let 臺 =
                連擊測試臺::建立(方案值, None, vec![字根目標("ㄓㄨˉ"), 字根目標("ㄍㄜˉ")]);

            // 擊 Z (`ㄓ`) 切入韻母盤
            臺.擊鍵(KeyCode::Z);
            assert_ne!(臺.機關.逐鍵提示.get(), None);
            // 擊 K (`ㄨ`) + V (`ˉ`) 完成首字, 自動回歸聲母盤並駐留
            臺.擊鍵(KeyCode::K);
            臺.擊鍵(KeyCode::V);
            assert!(臺.機關.連擊片段完成.get());
            assert_eq!(臺.機關.已錄入字根碼.get(), "ㄓㄨˉ");
            // 擊空格收結前字
            臺.擊鍵(KeyCode::Space);
            assert!(!臺.機關.連擊片段完成.get());
            assert_eq!(臺.機關.連擊對照.get().待輸後綴, "ㄍㄜˉ");
            let _ = 默認聲母盤;
        });
    }
}
