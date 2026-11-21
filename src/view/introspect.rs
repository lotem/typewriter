//! [內觀視圖] 純界面自動排版與 SVG 渲染組件:
//! 有向無環圖分層佈局, 過境光束走廊避讓, 面板四側接口自動分揀與三次貝塞爾光纖舞臺.

use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;

use crate::introspect::{
    九脈, 信號管線, 內觀總樞, 機關節點, 管線紐, 節點態勢, 節點紐, 響應角色
};

const 左列邊界: f32 = 115.0;
const 右列邊界: f32 = 1085.0;
const 面板寬度: f32 = 156.0;
const 面板高度: f32 = 74.0;
const 面板半寬: f32 = 面板寬度 * 0.5;
const 面板半高: f32 = 面板高度 * 0.5;
const 頂部縱界: f32 = 120.0;
const 中軸縱標: f32 = 300.0;
const 底部縱界: f32 = 480.0;
const 同列最小縱距: f32 = 155.0;
const 藍圖帶縱座標: f32 = 620.0;

/// 宣紙墨韻深色底配琥珀金, 松煙青與月白光束色系 (純視圖配色表).
const fn 九脈光束色(分組: 九脈) -> &'static str {
    match 分組 {
        九脈::觸鍵消息脈 => "#f5b942", // 琥珀金
        九脈::方案法度脈 => "#a8b8c8", // 蒼青月白
        九脈::盤面經緯脈 => "#7ec4cf", // 松煙青
        九脈::並擊和絃脈 => "#f28f3b", // 鎏金橙
        九脈::連擊字根脈 => "#5ec297", // 翡翠青
        九脈::習題碼表脈 => "#95a7ff", // 青金石
        九脈::反查指路脈 => "#b9e38d", // 柳芽青
        九脈::批閱步進脈 => "#ff7a6b", // 硃砂珊瑚
        九脈::走紙回顯脈 => "#f4efe6", // 宣紙月白
        九脈::待鑄藍圖脈 => "#6e6a63", // 古墨暗紋
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum 接口方位 {
    左,
    右,
    上,
    下,
}

#[derive(Clone, Copy, Debug, Default)]
struct 節點座標 {
    x: f32,
    y: f32,
    列: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
struct 列區標題 {
    x: f32,
    標題: &'static str,
}

#[derive(Clone, Copy, Debug)]
struct 面板接口 {
    管線: 管線紐,
    是否輸出: bool,
    光束色: &'static str,
    x: f32,
    y: f32,
}

#[derive(Clone, Debug, Default)]
struct 光纖幾何 {
    中點x: f32,
    中點y: f32,
    路徑: String,
}

struct 視圖拓撲幾何 {
    節點接口: Vec<Vec<面板接口>>,
    管線路徑: Vec<光纖幾何>,
}

struct 雙態視圖幾何 {
    節點座標: Vec<節點座標>,
    列標題: Vec<列區標題>,
    實裝: 視圖拓撲幾何,
    全量: 視圖拓撲幾何,
}

impl 雙態視圖幾何 {
    fn 構建(衆節點: &[機關節點], 衆管線: &[信號管線]) -> Self {
        let (節點座標, 列標題) = 自動排佈拓撲座標(衆節點, 衆管線);
        let 實裝 = 構建單態幾何(衆節點, 衆管線, &節點座標, false);
        let 全量 = 構建單態幾何(衆節點, 衆管線, &節點座標, true);
        Self {
            節點座標,
            列標題,
            實裝,
            全量,
        }
    }

    fn 查詢節點座標(&self, 紐: 節點紐) -> 節點座標 {
        self.節點座標.get(紐.0).copied().unwrap_or_default()
    }

    fn 查詢面板接口(&self, 紐: 節點紐, 含藍圖: bool) -> &[面板接口] {
        let 幾何 = if 含藍圖 {
            &self.全量
        } else {
            &self.實裝
        };
        幾何.節點接口.get(紐.0).map_or(&[], Vec::as_slice)
    }

    fn 查詢管線幾何(&self, 紐: 管線紐, 含藍圖: bool) -> Option<&光纖幾何> {
        let 幾何 = if 含藍圖 {
            &self.全量
        } else {
            &self.實裝
        };
        幾何.管線路徑.get(紐.0).filter(|路徑| !路徑.路徑.is_empty())
    }
}

// ============================================================================
// 一. 有向無環圖自動分層與過境走廊避讓排版算法
// ============================================================================

fn 推導列區標題(
    列序: usize,
    總列數: usize,
    列內節點: &[usize],
    衆節點: &[機關節點],
) -> &'static str {
    let 全屬 = |目標角色| {
        列內節點.iter().all(
            |&序| matches!(衆節點[序].態勢, 節點態勢::實裝 { 主角色, .. } if 主角色 == 目標角色),
        )
    };
    if 列序 == 0 {
        "〔 源頭與法度 〕"
    } else if 全屬(響應角色::閘) || 列序 + 1 == 總列數 {
        "〔 視圖大閘 〕"
    } else if 全屬(響應角色::流) {
        "〔 字幕走紙 〕"
    } else if 全屬(響應角色::澤) {
        if 列內節點.len() >= 2 {
            "〔 雙軌指法折射 〕"
        } else {
            "〔 蓄水樞紐 〕"
        }
    } else {
        "〔 盤面與作業 〕"
    }
}

fn 約束同列間距(縱座標組: &mut [f32]) {
    let 個數 = 縱座標組.len();
    if 個數 == 0 {
        return;
    }
    if 個數 == 1 {
        縱座標組[0] = 縱座標組[0].clamp(頂部縱界, 底部縱界);
        return;
    }

    let 間距 = 同列最小縱距.min((底部縱界 - 頂部縱界) / ((個數 - 1) as f32));
    for 輪 in 0..4 {
        for i in 0..(個數 - 1) {
            if 縱座標組[i + 1] - 縱座標組[i] < 間距 {
                let 中值 = 0.5 * (縱座標組[i] + 縱座標組[i + 1]);
                縱座標組[i] = 中值 - 0.5 * 間距;
                縱座標組[i + 1] = 中值 + 0.5 * 間距;
            }
        }
        if 縱座標組[0] < 頂部縱界 {
            let 補差 = 頂部縱界 - 縱座標組[0];
            if 輪 == 3 {
                縱座標組[0] = 頂部縱界;
                for i in 1..個數 {
                    縱座標組[i] = 縱座標組[i].max(縱座標組[i - 1] + 間距);
                }
            } else {
                for 值 in 縱座標組.iter_mut() {
                    *值 += 補差;
                }
            }
        }
        if let Some(&末值) = 縱座標組.last() {
            if 末值 > 底部縱界 {
                let 補差 = 末值 - 底部縱界;
                if 輪 == 3 {
                    縱座標組[個數 - 1] = 底部縱界;
                    for i in (0..(個數 - 1)).rev() {
                        縱座標組[i] = 縱座標組[i].min(縱座標組[i + 1] - 間距);
                    }
                } else {
                    for 值 in 縱座標組.iter_mut() {
                        *值 -= 補差;
                    }
                }
            }
        }
    }
}

fn 自動排佈拓撲座標(
    衆節點: &[機關節點],
    衆管線: &[信號管線],
) -> (Vec<節點座標>, Vec<列區標題>) {
    let 總數 = 衆節點.len();
    let mut 座標表 = vec![節點座標::default(); 總數];
    if 總數 == 0 {
        return (座標表, Vec::new());
    }

    // 1. 按純數據流依賴邊 (`角色 != 掣` 且非待鑄藍圖) 推導實裝節點的最長路徑層級
    let mut 原始層 = vec![0usize; 總數];
    let 實裝上限 = 衆節點.iter().filter(|節點| 節點.態勢.是否實裝()).count();
    for _ in 0..實裝上限 {
        let mut 有變更 = false;
        for 管線 in 衆管線 {
            if 管線.分組 == 九脈::待鑄藍圖脈 || 管線.角色 == 響應角色::掣 {
                continue;
            }
            let (源, 目標) = (管線.源節點.0, 管線.目標節點.0);
            if 源 == 目標 || !衆節點[源].態勢.是否實裝() || !衆節點[目標].態勢.是否實裝()
            {
                continue;
            }
            if 原始層[目標] < 原始層[源] + 1 {
                原始層[目標] = 原始層[源] + 1;
                有變更 = true;
            }
        }
        if !有變更 {
            break;
        }
    }

    // 識別視圖匯聚 `[閘]` 節點與純閘前展示 `[流]` 節點 (出邊全部直通視圖閘門)
    let mut 是大閘 = vec![false; 總數];
    let mut 是閘前流 = vec![false; 總數];
    for (序, 節點) in 衆節點.iter().enumerate() {
        if let 節點態勢::實裝 { 主角色, .. } = 節點.態勢 {
            if 主角色 == 響應角色::閘 {
                是大閘[序] = true;
            } else if 主角色 == 響應角色::流 {
                let mut 出邊數 = 0usize;
                let mut 皆入閘 = true;
                for 管線 in 衆管線 {
                    if 管線.分組 != 九脈::待鑄藍圖脈 && 管線.源節點.0 == 序 {
                        出邊數 += 1;
                        if 管線.角色 != 響應角色::閘 {
                            皆入閘 = false;
                        }
                    }
                }
                是閘前流[序] = 出邊數 > 0 && 皆入閘;
            }
        }
    }

    let 最大骨幹層 = (0..總數)
        .filter(|&序| 衆節點[序].態勢.是否實裝() && !是大閘[序] && !是閘前流[序])
        .map(|序| 原始層[序])
        .max()
        .unwrap_or(0);

    for 序 in 0..總數 {
        if 是閘前流[序] {
            let 同層有骨幹 = (0..總數).any(|他| {
                他 != 序
                    && 衆節點[他].態勢.是否實裝()
                    && !是大閘[他]
                    && !是閘前流[他]
                    && 原始層[他] == 原始層[序]
            });
            if 同層有骨幹 {
                原始層[序] = 最大骨幹層 + 1;
            }
        }
    }

    let 最大非閘層 = (0..總數)
        .filter(|&序| 衆節點[序].態勢.是否實裝() && !是大閘[序])
        .map(|序| 原始層[序])
        .max()
        .unwrap_or(0);
    for 序 in 0..總數 {
        if 是大閘[序] {
            原始層[序] = 最大非閘層 + 1;
        }
    }

    // 壓縮非空層級爲緊湊列號 `0..總列數`
    let mut 出現層: Vec<usize> = (0..總數)
        .filter(|&序| 衆節點[序].態勢.是否實裝())
        .map(|序| 原始層[序])
        .collect();
    出現層.sort_unstable();
    出現層.dedup();

    let 總列數 = 出現層.len();
    let mut 各列節點: Vec<Vec<usize>> = vec![Vec::new(); 總列數];
    for (序, 節點) in 衆節點.iter().enumerate() {
        if 節點.態勢.是否實裝() {
            if let Ok(列號) = 出現層.binary_search(&原始層[序]) {
                座標表[序].列 = Some(列號);
                各列節點[列號].push(序);
            }
        }
    }

    // 2. 分配各列橫座標 `x` 與初值縱座標 `y`
    let mut 列座標x = vec![600.0; 總列數];
    for (列號, x槽) in 列座標x.iter_mut().enumerate() {
        *x槽 = if 總列數 <= 1 {
            600.0
        } else {
            左列邊界 + (列號 as f32) * (右列邊界 - 左列邊界) / ((總列數 - 1) as f32)
        };
    }

    for (列號, 列節點) in 各列節點.iter().enumerate() {
        let 個數 = 列節點.len();
        for (位次, &節點序) in 列節點.iter().enumerate() {
            座標表[節點序].x = 列座標x[列號];
            座標表[節點序].y = if 個數 <= 1 {
                中軸縱標
            } else if 列號 == 0 || 列號 + 1 == 總列數 {
                頂部縱界 + (位次 as f32) * (底部縱界 - 頂部縱界) / ((個數 - 1) as f32)
            } else {
                中軸縱標 + ((位次 as f32) - ((個數 - 1) as f32) * 0.5) * 同列最小縱距
            };
        }
    }

    // 3. 鄰域重心吸引 + 過境光束走廊排斥迭代求解 (令中段機關自動讓開跨列光束通道)
    if 總列數 > 2 {
        for _ in 0..4 {
            for 列號 in 1..(總列數 - 1) {
                let x當前 = 列座標x[列號];
                let mut 過境束y = Vec::new();
                for 管線 in 衆管線 {
                    if 管線.分組 == 九脈::待鑄藍圖脈 {
                        continue;
                    }
                    let (源, 目標) = (管線.源節點.0, 管線.目標節點.0);
                    let (Some(源列), Some(目標列)) = (座標表[源].列, 座標表[目標].列)
                    else {
                        continue;
                    };
                    // 排除走畫布頂端天際長拱 (`dx < -600`) 的超遠距回傳線
                    if 座標表[目標].x - 座標表[源].x < -600.0 {
                        continue;
                    }
                    if 源列.min(目標列) < 列號 && 列號 < 源列.max(目標列) {
                        let 比例 = (x當前 - 座標表[源].x) / (座標表[目標].x - 座標表[源].x);
                        let 插值y = 座標表[源].y + (座標表[目標].y - 座標表[源].y) * 比例;
                        過境束y.push(插值y);
                    }
                }

                let 列節點數 = 各列節點[列號].len();
                let mut 提案: Vec<(usize, f32)> = 各列節點[列號]
                    .iter()
                    .map(|&節點序| {
                        let mut 鄰和 = 0.0f32;
                        let mut 鄰權 = 0.0f32;
                        for 管線 in 衆管線 {
                            if 管線.分組 == 九脈::待鑄藍圖脈 {
                                continue;
                            }
                            let 對端 = if 管線.源節點.0 == 節點序 {
                                Some(管線.目標節點.0)
                            } else if 管線.目標節點.0 == 節點序 {
                                Some(管線.源節點.0)
                            } else {
                                None
                            };
                            if let Some(對端序) = 對端 {
                                if 座標表[對端序].列.is_some_and(|對列| 對列 != 列號) {
                                    鄰和 += 座標表[對端序].y;
                                    鄰權 += 1.0;
                                }
                            }
                        }
                        let 重心y = if 鄰權 > 0.0 {
                            鄰和 / 鄰權
                        } else {
                            座標表[節點序].y
                        };

                        let 目標y = if 列節點數 == 1 && !過境束y.is_empty() {
                            // 單節點列整體避開過境光束走廊中軸, 絕不夾在多股平行光束縫隙中央
                            let 走廊中心 =
                                過境束y.iter().copied().sum::<f32>() / (過境束y.len() as f32);
                            let 方向 = if 重心y >= 走廊中心 { 1.0 } else { -1.0 };
                            let 避讓界 = 走廊中心 + 方向 * 118.0;
                            if 方向 > 0.0 {
                                重心y.max(避讓界)
                            } else {
                                重心y.min(避讓界)
                            }
                        } else {
                            let mut 推演y = 重心y;
                            for &過境y in &過境束y {
                                let 差 = 推演y - 過境y;
                                if 差.abs() < 95.0 {
                                    let 符號 = if 差.abs() < 1.0 {
                                        if 推演y >= 中軸縱標 {
                                            1.0
                                        } else {
                                            -1.0
                                        }
                                    } else {
                                        差.signum()
                                    };
                                    推演y += 符號 * (95.0 - 差.abs()) * 0.34;
                                }
                            }
                            推演y
                        };
                        (節點序, 目標y)
                    })
                    .collect();

                提案.sort_by(|甲, 乙| {
                    甲.1
                        .partial_cmp(&乙.1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| 甲.0.cmp(&乙.0))
                });

                let mut 縱列: Vec<f32> = 提案.iter().map(|&(_, y)| y).collect();
                約束同列間距(&mut 縱列);

                各列節點[列號].clear();
                for ((節點序, _), 修正y) in 提案.into_iter().zip(縱列) {
                    各列節點[列號].push(節點序);
                    座標表[節點序].y = 修正y;
                }
            }
        }
    }

    let 列標題 = 各列節點
        .iter()
        .enumerate()
        .map(|(列號, 列節點)| 列區標題 {
            x: 列座標x[列號],
            標題: 推導列區標題(列號, 總列數, 列節點, 衆節點),
        })
        .collect();

    // 4. 底部 [待鑄藍圖] 演進帶按拓撲序橫向排佈
    let mut 待鑄列表: Vec<usize> = (0..總數)
        .filter(|&序| !衆節點[序].態勢.是否實裝())
        .collect();
    if !待鑄列表.is_empty() {
        let mut 藍圖層 = vec![0usize; 總數];
        for _ in 0..待鑄列表.len() {
            for 管線 in 衆管線 {
                if 管線.分組 == 九脈::待鑄藍圖脈 {
                    let (源, 目標) = (管線.源節點.0, 管線.目標節點.0);
                    if !衆節點[源].態勢.是否實裝()
                        && !衆節點[目標].態勢.是否實裝()
                        && 藍圖層[目標] < 藍圖層[源] + 1
                    {
                        藍圖層[目標] = 藍圖層[源] + 1;
                    }
                }
            }
        }
        待鑄列表.sort_by_key(|&序| (藍圖層[序], 序));
        let 待鑄數 = 待鑄列表.len();
        for (位次, 節點序) in 待鑄列表.into_iter().enumerate() {
            座標表[節點序].x = 780.0 + ((位次 as f32) - ((待鑄數 - 1) as f32) * 0.5) * 180.0;
            座標表[節點序].y = 藍圖帶縱座標;
        }
    }

    (座標表, 列標題)
}

// ============================================================================
// 二. 面板四側接口分揀與生成式三次貝塞爾光纖曲線
// ============================================================================

fn 判定管線接口方位(
    管線: &信號管線,
    衆節點: &[機關節點],
    座標表: &[節點座標],
) -> (接口方位, 接口方位) {
    let 源 = 座標表[管線.源節點.0];
    let 目標 = 座標表[管線.目標節點.0];
    let dx = 目標.x - 源.x;
    let dy = 目標.y - 源.y;

    // 同列垂直管線
    if dx.abs() < 45.0 {
        return if dy >= 0.0 {
            (接口方位::下, 接口方位::上)
        } else {
            (接口方位::上, 接口方位::下)
        };
    }

    // 正向數據流與陡峭跨帶連接 (`dx >= 45.0`)
    if dx >= 45.0 {
        if dx < 120.0 && dy < -180.0 {
            return (接口方位::上, 接口方位::下);
        }
        if dx < 120.0 && dy > 180.0 {
            let 同列下方有面板 = 源.列.is_some_and(|源列| {
                衆節點.iter().enumerate().any(|(他序, 他節點)| {
                    他序 != 管線.源節點.0
                        && 他節點.態勢.是否實裝()
                        && 座標表[他序].列 == Some(源列)
                        && 座標表[他序].y > 源.y + 40.0
                })
            });
            if !同列下方有面板 {
                return (接口方位::下, 接口方位::上);
            }
        }
        return (接口方位::右, 接口方位::左);
    }

    // 反向回授控制閉環 (`dx <= -45.0`)
    if dx < -600.0 {
        return (接口方位::上, 接口方位::上);
    }
    let 源頂 = 源.y - 面板半高;
    let 源底 = 源.y + 面板半高;
    let 目標頂 = 目標.y - 面板半高;
    let 目標底 = 目標.y + 面板半高;

    if 源頂 > 目標底 + 70.0 {
        (接口方位::上, 接口方位::下)
    } else if 目標頂 > 源底 + 70.0 {
        (接口方位::下, 接口方位::上)
    } else if dy <= 0.0 {
        (接口方位::上, 接口方位::上)
    } else {
        (接口方位::下, 接口方位::下)
    }
}

fn 自動計算管線曲率(
    衆管線: &[信號管線], 座標表: &[節點座標], 含藍圖: bool
) -> Vec<f32> {
    let mut 曲率表 = vec![0.0f32; 衆管線.len()];

    for (序號, 管線) in 衆管線.iter().enumerate() {
        if !含藍圖 && 管線.分組 == 九脈::待鑄藍圖脈 {
            continue;
        }
        let mut 同束總數 = 0usize;
        let mut 束內位次 = 0usize;
        for (他序, 他線) in 衆管線.iter().enumerate() {
            if !含藍圖 && 他線.分組 == 九脈::待鑄藍圖脈 {
                continue;
            }
            if 他線.源節點 == 管線.源節點 && 他線.目標節點 == 管線.目標節點 {
                if 他序 < 序號 {
                    束內位次 += 1;
                }
                同束總數 += 1;
            }
        }

        // 1. 同源同匯平行信號束如琴絃般對稱散開
        let 同束散開 = if 同束總數 <= 1 {
            0.0
        } else {
            ((束內位次 as f32) - ((同束總數 - 1) as f32) * 0.5) * 26.0
        };

        // 2. 跨越中間列的長程正向管線自動向上/下外拱, 繞開中段機關面板
        let 源 = 座標表[管線.源節點.0];
        let 目標 = 座標表[管線.目標節點.0];
        let dx = 目標.x - 源.x;
        let 跨列外拱 = if dx > 360.0 {
            let 跨度比 = (dx - 260.0) / 480.0;
            if 源.y < 220.0 {
                -66.0 * 跨度比
            } else if 源.y > 380.0 {
                72.0 * 跨度比
            } else {
                0.0
            }
        } else {
            0.0
        };

        曲率表[序號] = 同束散開 + 跨列外拱;
    }

    曲率表
}

fn 計算單側面板接口(
    節點標識: 節點紐,
    方位: 接口方位,
    衆節點: &[機關節點],
    衆管線: &[信號管線],
    座標表: &[節點座標],
    曲率表: &[f32],
    含藍圖: bool,
) -> Vec<面板接口> {
    let 節點 = 座標表[節點標識.0];
    let mut 關聯管線: Vec<(&信號管線, bool, f32)> = 衆管線
        .iter()
        .filter(|管線| 含藍圖 || 管線.分組 != 九脈::待鑄藍圖脈)
        .filter_map(|管線| {
            let (穿出側, 匯入側) = 判定管線接口方位(管線, 衆節點, 座標表);
            let 曲率 = 曲率表[管線.紐.0];
            if 管線.源節點 == 節點標識 && 穿出側 == 方位 {
                let 對端 = 座標表[管線.目標節點.0];
                let 排序鍵 = match 方位 {
                    接口方位::左 | 接口方位::右 => 對端.y + 曲率 * 0.5,
                    接口方位::上 | 接口方位::下 => 對端.x + 曲率 * 0.5,
                };
                Some((管線, true, 排序鍵))
            } else if 管線.目標節點 == 節點標識 && 匯入側 == 方位 {
                let 對端 = 座標表[管線.源節點.0];
                let 排序鍵 = match 方位 {
                    接口方位::左 | 接口方位::右 => 對端.y + 曲率 * 0.5,
                    接口方位::上 | 接口方位::下 => 對端.x + 曲率 * 0.5,
                };
                Some((管線, false, 排序鍵))
            } else {
                None
            }
        })
        .collect();

    關聯管線.sort_by(|甲, 乙| {
        甲.2
            .partial_cmp(&乙.2)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| 甲.0.紐.cmp(&乙.0.紐))
    });

    let 總數 = 關聯管線.len();
    關聯管線
        .into_iter()
        .enumerate()
        .map(|(序號, (管線, 是否輸出, _))| {
            let 偏移 = if 總數 <= 1 {
                0.0
            } else {
                let 歸一 = (序號 as f32) / ((總數 - 1) as f32) - 0.5;
                match 方位 {
                    接口方位::左 | 接口方位::右 => {
                        let 跨度 = ((總數 - 1) as f32 * 10.5).min(42.0);
                        歸一 * 跨度
                    }
                    接口方位::上 | 接口方位::下 => {
                        let 跨度 = ((總數 - 1) as f32 * 18.0).min(96.0);
                        歸一 * 跨度
                    }
                }
            };

            let (x, y) = match 方位 {
                接口方位::左 => (節點.x - 面板半寬, 節點.y + 6.0 + 偏移),
                接口方位::右 => (節點.x + 面板半寬, 節點.y + 6.0 + 偏移),
                接口方位::上 => (節點.x + 偏移, 節點.y - 面板半高),
                接口方位::下 => (節點.x + 偏移, 節點.y + 面板半高),
            };

            面板接口 {
                管線: 管線.紐,
                是否輸出,
                光束色: 九脈光束色(管線.分組),
                x,
                y,
            }
        })
        .collect()
}

fn 構建單態幾何(
    衆節點: &[機關節點],
    衆管線: &[信號管線],
    座標表: &[節點座標],
    含藍圖: bool,
) -> 視圖拓撲幾何 {
    let 曲率表 = 自動計算管線曲率(衆管線, 座標表, 含藍圖);

    let 節點接口: Vec<Vec<面板接口>> = 衆節點
        .iter()
        .map(|節點| {
            let mut 全部接口 = Vec::with_capacity(8);
            for 方位 in [接口方位::左, 接口方位::右, 接口方位::上, 接口方位::下]
            {
                全部接口.extend(計算單側面板接口(
                    節點.紐, 方位, 衆節點, 衆管線, 座標表, &曲率表, 含藍圖,
                ));
            }
            全部接口
        })
        .collect();

    let 管線路徑: Vec<光纖幾何> = 衆管線
        .iter()
        .map(|管線| {
            if !含藍圖 && 管線.分組 == 九脈::待鑄藍圖脈 {
                return 光纖幾何::default();
            }
            let 源 = 座標表[管線.源節點.0];
            let 目標 = 座標表[管線.目標節點.0];
            let (穿出側, 匯入側) = 判定管線接口方位(管線, 衆節點, 座標表);

            let (起x, 起y) = 節點接口[管線.源節點.0]
                .iter()
                .find(|接口| 接口.管線 == 管線.紐 && 接口.是否輸出)
                .map_or((源.x, 源.y), |接口| (接口.x, 接口.y));
            let (止x, 止y) = 節點接口[管線.目標節點.0]
                .iter()
                .find(|接口| 接口.管線 == 管線.紐 && !接口.是否輸出)
                .map_or((目標.x, 目標.y), |接口| (接口.x, 接口.y));

            let (控一x, 控一y, 控二x, 控二y) =
                if 穿出側 == 接口方位::上 && 匯入側 == 接口方位::上 {
                    if 目標.x - 源.x < -600.0 {
                        // 跨全屏觸擊回傳: 沿畫布天際長拱回到輸入源頂端接口
                        (起x - 120.0, 26.0, 止x + 120.0, 26.0)
                    } else {
                        // 局部上拱回授閉環
                        let 上拱 = (起y.min(止y) - 52.0).max(34.0);
                        (起x - 36.0, 上拱, 止x + 36.0, 上拱)
                    }
                } else if 穿出側 == 接口方位::下 && 匯入側 == 接口方位::下 {
                    // 局部下探回授閉環
                    let 下探 = 起y.max(止y) + 56.0;
                    (起x - 36.0, 下探, 止x + 36.0, 下探)
                } else {
                    let dx = 止x - 起x;
                    let dy = 止y - 起y;
                    let 距離 = (dx * dx + dy * dy).sqrt().max(1.0);
                    let 切線長 = (距離 * 0.38).clamp(44.0, 175.0);

                    let (出法x, 出法y) = match 穿出側 {
                        接口方位::右 => (1.0, 0.0),
                        接口方位::左 => (-1.0, 0.0),
                        接口方位::下 => (0.0, 1.0),
                        接口方位::上 => (0.0, -1.0),
                    };
                    let (入法x, 入法y) = match 匯入側 {
                        接口方位::左 => (-1.0, 0.0),
                        接口方位::右 => (1.0, 0.0),
                        接口方位::上 => (0.0, -1.0),
                        接口方位::下 => (0.0, 1.0),
                    };

                    let 法線x = -dy / 距離;
                    let 法線y = dx / 距離;
                    let 曲率 = 曲率表[管線.紐.0] * 0.28;

                    (
                        起x + 出法x * 切線長 + 法線x * 曲率,
                        起y + 出法y * 切線長 + 法線y * 曲率,
                        止x + 入法x * 切線長 + 法線x * 曲率,
                        止y + 入法y * 切線長 + 法線y * 曲率,
                    )
                };

            let 中點x = 0.125 * 起x + 0.375 * 控一x + 0.375 * 控二x + 0.125 * 止x;
            let 中點y = 0.125 * 起y + 0.375 * 控一y + 0.375 * 控二y + 0.125 * 止y;
            let 路徑 = format!(
                "M {起x:.1} {起y:.1} C {控一x:.1} {控一y:.1}, {控二x:.1} {控二y:.1}, {止x:.1} {止y:.1}"
            );

            光纖幾何 { 中點x, 中點y, 路徑 }
        })
        .collect();

    視圖拓撲幾何 {
        節點接口, 管線路徑
    }
}

// ============================================================================
// 三. Leptos 內觀視圖渲染組件
// ============================================================================

fn 釋放按鈕焦點(事件: &leptos::ev::MouseEvent) {
    if let Some(元素) = 事件
        .current_target()
        .and_then(|目標| 目標.dyn_into::<web_sys::HtmlElement>().ok())
    {
        let _ = 元素.blur();
    }
}

/// 渲染單條光纖靜態外鞘與底芯 (位於機關面板下層).
#[component]
fn Rime光纖靜態管線(
    管線: 信號管線,
    內觀: 內觀總樞,
    幾何緩存: StoredValue<雙態視圖幾何>,
) -> impl IntoView {
    let 管線態信號 = 內觀.管線信號(管線.紐);
    let 光束色 = 九脈光束色(管線.分組);
    let 是否藍圖 = 管線.分組 == 九脈::待鑄藍圖脈;

    view! {
        <Show when=move || !是否藍圖 || 內觀.是否顯示藍圖()>
            {move || {
                let 含藍圖 = 內觀.是否顯示藍圖();
                let Some((路徑, 中點x, 中點y)) = 幾何緩存.with_value(|緩存| {
                    緩存.查詢管線幾何(管線.紐, 含藍圖)
                        .map(|幾何| (幾何.路徑.clone(), 幾何.中點x, 幾何.中點y))
                }) else {
                    return ().into_any();
                };

                let 已接線 = 管線態信號.is_some_and(|信號| 信號.with(|狀態| 狀態.已接線));
                let 活躍 = 管線態信號.is_some_and(|信號| 信號.with(|狀態| 狀態.活躍));

                let 九脈濾鏡 = 內觀.聚光九脈.get();
                let 角色濾鏡 = 內觀.聚光角色.get();
                let 聚光選中 = 九脈濾鏡 == Some(管線.分組) || 角色濾鏡 == Some(管線.角色);
                let 暗化 = (九脈濾鏡.is_some() && 九脈濾鏡 != Some(管線.分組))
                    || (角色濾鏡.is_some() && 角色濾鏡 != Some(管線.角色));

                let 虛線樣式 = if !已接線 || 是否藍圖 {
                    "6 5"
                } else if 管線.角色 == 響應角色::掣 {
                    "3 3"
                } else {
                    "none"
                };

                view! {
                    <g
                        class="xray-fiber-group"
                        class:is-wired=已接線
                        class:is-unwired=!已接線
                        class:is-active=活躍
                        class:is-spotlight=聚光選中
                        class:is-dimmed=暗化
                    >
                        <path
                            class="xray-fiber-sheath"
                            d=路徑.clone()
                            fill="none"
                        />
                        <path
                            class="xray-fiber-core"
                            d=路徑
                            fill="none"
                            stroke=光束色
                            stroke-dasharray=虛線樣式
                        />
                        {聚光選中.then(|| {
                            let 標籤 = format!("{} {}", 管線.角色.符號(), 管線.信號名);
                            let 標籤寬 = (標籤.chars().count() as f32 * 6.8 + 16.0).max(68.0);
                            view! {
                                <g transform=format!("translate({中點x:.1}, {中點y:.1})")>
                                    <rect
                                        x=format!("{:.1}", -標籤寬 * 0.5)
                                        y="-10"
                                        width=format!("{標籤寬:.1}")
                                        height="19"
                                        rx="9.5"
                                        class="xray-edge-pill spotlight-pill"
                                        stroke=光束色
                                    />
                                    <text y="3.5" text-anchor="middle" class="xray-edge-pill-text">
                                        {標籤}
                                    </text>
                                </g>
                            }
                        })}
                    </g>
                }.into_any()
            }}
        </Show>
    }
}

/// 渲染事件觸發時沿光纖向下游飛流而去的純能光束 (位於機關面板上層, 按 `世代` 精確重播).
#[component]
fn Rime光纖純能光束(
    管線: 信號管線,
    內觀: 內觀總樞,
    幾何緩存: StoredValue<雙態視圖幾何>,
) -> impl IntoView {
    let Some(管線態信號) = 內觀.管線信號(管線.紐) else {
        return ().into_any();
    };
    let 光束色 = 九脈光束色(管線.分組);

    view! {
        {move || {
            let 狀態 = 管線態信號.get();
            if !狀態.活躍 || 狀態.世代 == 0 {
                return None;
            }
            let 含藍圖 = 內觀.是否顯示藍圖_不追蹤();
            let (路徑, 中點x, 中點y) = 幾何緩存.with_value(|緩存| {
                緩存.查詢管線幾何(管線.紐, 含藍圖)
                    .map(|幾何| (幾何.路徑.clone(), 幾何.中點x, 幾何.中點y))
            })?;
            let 膠囊文本 = if 狀態.摘要.is_empty() {
                format!("{} {}", 管線.角色.符號(), 管線.信號名)
            } else {
                format!("{} {}", 管線.角色.符號(), 狀態.摘要)
            };
            let 膠囊寬 = (膠囊文本.chars().count() as f32 * 7.2 + 18.0).clamp(58.0, 196.0);

            Some(view! {
                <g class="xray-active-beam-instance" data-seq=狀態.世代>
                    <path
                        class="xray-fiber-afterglow"
                        d=路徑.clone()
                        fill="none"
                        stroke=光束色
                        filter="url(#xray-beam-glow)"
                    />
                    <path
                        class="xray-energy-aura"
                        d=路徑.clone()
                        pathLength="100"
                        fill="none"
                        stroke=光束色
                        filter="url(#xray-beam-glow)"
                    />
                    <path
                        class="xray-energy-core"
                        d=路徑.clone()
                        pathLength="100"
                        fill="none"
                    />
                    <circle r="3.5" class="xray-photon-head" filter="url(#xray-beam-glow)">
                        <animateMotion
                            dur="0.38s"
                            repeatCount="1"
                            fill="freeze"
                            path=路徑
                            calcMode="spline"
                            keyTimes="0;1"
                            keyPoints="0;1"
                            keySplines="0.18 0.82 0.28 1"
                        />
                    </circle>
                    <g
                        class="xray-beam-caption"
                        transform=format!("translate({中點x:.1}, {:.1})", 中點y - 2.0)
                    >
                        <rect
                            x=format!("{:.1}", -膠囊寬 * 0.5)
                            y="-10.5"
                            width=format!("{膠囊寬:.1}")
                            height="20"
                            rx="10"
                            class="xray-edge-pill"
                            stroke=光束色
                        />
                        <text y="3.5" text-anchor="middle" class="xray-edge-pill-text">
                            {膠囊文本}
                        </text>
                    </g>
                </g>
            })
        }}
    }
    .into_any()
}

/// 渲染單個機關組件面板 (含邊框物理光纖接口, 水文角色印記與實時數據讀數窗).
#[component]
fn Rime機關面板(
    節點: 機關節點,
    內觀: 內觀總樞,
    幾何緩存: StoredValue<雙態視圖幾何>,
) -> impl IntoView {
    let 節點態信號 = 內觀.節點信號(節點.紐);
    let 座標 = 幾何緩存.with_value(|緩存| 緩存.查詢節點座標(節點.紐));
    let 左緣 = 座標.x - 面板半寬;
    let 上緣 = 座標.y - 面板半高;
    let 是否實裝 = 節點.態勢.是否實裝();

    view! {
        <Show when=move || 是否實裝 || 內觀.是否顯示藍圖()>
            {move || {
                let 含藍圖 = 內觀.是否顯示藍圖();
                let 衆接口 = 幾何緩存.with_value(|緩存| 緩存.查詢面板接口(節點.紐, 含藍圖).to_vec());
                let 角色濾鏡 = 內觀.聚光角色.get();

                let 節點態 = 節點態信號.map(|信號| 信號.get()).unwrap_or_default();
                let 活躍 = 節點態.活躍;

                let (角色印章, 模塊說明, 角色匹配) = match 節點.態勢 {
                    節點態勢::實裝 { 模塊, 主角色 } => (
                        format!("{} {}", 主角色.符號(), 主角色.名稱()),
                        模塊,
                        角色濾鏡.is_none_or(|選中| 選中 == 主角色),
                    ),
                    節點態勢::待鑄 { 設想職責 } => ("待鑄".to_string(), 設想職責, 角色濾鏡.is_none()),
                };

                let 讀數文本 = if !是否實裝 {
                    "[七境演進預留]".to_string()
                } else if !節點態.快照.is_empty() {
                    節點態.快照.clone()
                } else {
                    format!("已接 {} 脈 · 待命", 節點態.已接管線數)
                };

                view! {
                    <g
                        class="xray-gear-panel"
                        class:is-live=是否實裝
                        class:is-blueprint=!是否實裝
                        class:is-active=活躍
                        class:is-dimmed=!角色匹配
                    >
                        {活躍.then(|| view! {
                            <rect
                                x=format!("{:.1}", 左緣 - 2.5)
                                y=format!("{:.1}", 上緣 - 2.5)
                                width=format!("{:.1}", 面板寬度 + 5.0)
                                height=format!("{:.1}", 面板高度 + 5.0)
                                rx="9.5"
                                class="xray-panel-halo"
                                filter="url(#xray-node-glow)"
                            />
                        })}

                        <rect
                            x=format!("{左緣:.1}")
                            y=format!("{上緣:.1}")
                            width=format!("{面板寬度:.1}")
                            height=format!("{面板高度:.1}")
                            rx="7.5"
                            class="xray-panel-body"
                        />

                        <rect
                            x=format!("{:.1}", 左緣 + 1.0)
                            y=format!("{:.1}", 上緣 + 1.0)
                            width=format!("{:.1}", 面板寬度 - 2.0)
                            height="21"
                            rx="6.5"
                            class="xray-panel-header"
                        />

                        <text
                            x=format!("{:.1}", 左緣 + 10.0)
                            y=format!("{:.1}", 上緣 + 15.0)
                            class="xray-panel-title"
                        >
                            {節點.標題}
                        </text>
                        <text
                            x=format!("{:.1}", 左緣 + 面板寬度 - 8.0)
                            y=format!("{:.1}", 上緣 + 14.5)
                            text-anchor="end"
                            class="xray-panel-role-tag"
                        >
                            {角色印章}
                        </text>

                        <text
                            x=format!("{:.1}", 左緣 + 10.0)
                            y=format!("{:.1}", 上緣 + 34.5)
                            class="xray-panel-subtitle"
                        >
                            {模塊說明}
                        </text>

                        <rect
                            x=format!("{:.1}", 左緣 + 8.0)
                            y=format!("{:.1}", 上緣 + 41.0)
                            width=format!("{:.1}", 面板寬度 - 16.0)
                            height="25"
                            rx="4"
                            class="xray-panel-readout-box"
                        />
                        <text
                            x=format!("{:.1}", 座標.x)
                            y=format!("{:.1}", 上緣 + 57.5)
                            text-anchor="middle"
                            class="xray-panel-readout-text"
                        >
                            {讀數文本}
                        </text>

                        {衆接口.into_iter().map(|接口| {
                            let 接口管線信號 = 內觀.管線信號(接口.管線);
                            let 接口活躍 = move || {
                                接口管線信號.is_some_and(|信號| 信號.with(|狀態| 狀態.活躍))
                            };
                            view! {
                                <g
                                    class="xray-port-socket"
                                    class:is-output=接口.是否輸出
                                    class:is-input=!接口.是否輸出
                                    class:is-active=接口活躍
                                >
                                    <circle
                                        cx=format!("{:.1}", 接口.x)
                                        cy=format!("{:.1}", 接口.y)
                                        r="4.2"
                                        class="xray-port-collar"
                                    />
                                    <circle
                                        cx=format!("{:.1}", 接口.x)
                                        cy=format!("{:.1}", 接口.y)
                                        r=move || if 接口活躍() { "3.0" } else { "1.9" }
                                        fill=move || if 接口活躍() { "#fffdf5" } else { 接口.光束色 }
                                        class="xray-port-core"
                                    />
                                </g>
                            }
                        }).collect_view()}
                    </g>
                }
            }}
        </Show>
    }
}

/// [內觀視圖] 頂部九脈光束色標聚光燈欄.
#[component]
pub fn Rime九脈欄(內觀: 內觀總樞) -> impl IntoView {
    view! {
        <Show when=move || 內觀.開啓.get() && !內觀.錄屏模式.get()>
            <nav class="xray-conduit-strip">
                {九脈::實裝九脈.into_iter().map(|分組| {
                    let 光束色 = 九脈光束色(分組);
                    let 已選中 = move || 內觀.聚光九脈.get() == Some(分組);
                    view! {
                        <button
                            type="button"
                            tabindex="-1"
                            class="xray-conduit-chip"
                            class:active=已選中
                            on:mousedown=move |事件| 事件.prevent_default()
                            on:mouseenter=move |_| 內觀.聚光九脈.set(Some(分組))
                            on:mouseleave=move |_| 內觀.聚光九脈.set(None)
                            on:click=move |事件| {
                                釋放按鈕焦點(&事件);
                                內觀.聚光九脈.update(|當前| {
                                    *當前 = if *當前 == Some(分組) { None } else { Some(分組) };
                                });
                            }
                        >
                            <span
                                class="xray-conduit-dot"
                                style=format!("background-color: {光束色}; box-shadow: 0 0 6px {光束色};")
                            ></span>
                            <span>{分組.名稱()}</span>
                        </button>
                    }
                }).collect_view()}
            </nav>
        </Show>
    }
}

/// 機關面板與光纖純能光束 SVG 舞臺 (由邏輯拓撲自動生成分層排版與光纖幾何).
#[component]
pub fn Rime內觀圖(內觀: 內觀總樞) -> impl IntoView {
    let (衆節點, 衆管線, 雙態幾何) = 內觀.借閱拓撲(|節點表, 管線表| {
        (
            節點表.to_vec(),
            管線表.to_vec(),
            雙態視圖幾何::構建(節點表, 管線表),
        )
    });
    let 列標題組 = 雙態幾何.列標題.clone();
    let 幾何緩存 = StoredValue::new(雙態幾何);

    let 視口框 = move || {
        if 內觀.是否顯示藍圖() {
            "0 0 1200 680"
        } else {
            // 收起待鑄藍圖帶時裁剪底部空白, 令九大實裝機關放大並爲下方縮微鍵盤騰出舒展空間
            "0 0 1200 538"
        }
    };

    view! {
        <div class="xray-canvas-wrap">
            <svg
                class="xray-svg"
                viewBox=視口框
                preserveAspectRatio="xMidYMid meet"
            >
                <defs>
                    <filter id="xray-beam-glow" x="-35%" y="-35%" width="170%" height="170%">
                        <feGaussianBlur stdDeviation="3.2" result="blur" />
                        <feMerge>
                            <feMergeNode in="blur" />
                            <feMergeNode in="SourceGraphic" />
                        </feMerge>
                    </filter>
                    <filter id="xray-node-glow" x="-25%" y="-25%" width="150%" height="150%">
                        <feGaussianBlur stdDeviation="5.5" result="halo" />
                        <feMerge>
                            <feMergeNode in="halo" />
                            <feMergeNode in="SourceGraphic" />
                        </feMerge>
                    </filter>
                    <radialGradient id="xray-chassis-vignette" cx="50%" cy="46%" r="60%">
                        <stop offset="0%" stop-color="#201d19" stop-opacity="0.55" />
                        <stop offset="100%" stop-color="#0b0a09" stop-opacity="0.0" />
                    </radialGradient>
                </defs>

                <g class="xray-chassis-layer">
                    <rect x="0" y="0" width="1200" height="680" fill="url(#xray-chassis-vignette)" />

                    {列標題組.into_iter().map(|列標| view! {
                        <text
                            x=format!("{:.1}", 列標.x)
                            y="54"
                            text-anchor="middle"
                            class="xray-zone-label"
                        >
                            {列標.標題}
                        </text>
                    }).collect_view()}

                    <Show when=move || 內觀.是否顯示藍圖()>
                        <line
                            x1="480" y1="560" x2="1080" y2="560"
                            stroke="#3a342c" stroke-width="1" stroke-dasharray="4 6"
                        />
                        <text x="480" y="552" class="xray-blueprint-banner">
                            "七境演進 · 待鑄藍圖預留位 (未實裝模塊, 不參與實況合詠)"
                        </text>
                    </Show>

                    {move || {
                        let 共振序號 = 內觀.合詠世代.get();
                        (內觀.合詠活躍.get() && 共振序號 > 0).then(|| view! {
                            <g class="xray-resonance-group" data-rseq=共振序號>
                                <circle cx="600" cy="295" r="40" class="xray-resonance-ring ring-1" />
                                <circle cx="600" cy="295" r="40" class="xray-resonance-ring ring-2" />
                                <text x="600" y="34" text-anchor="middle" class="xray-resonance-banner">
                                    "✦ 四階貫通 · 一擊合詠 ✦"
                                </text>
                            </g>
                        })
                    }}
                </g>

                <g class="xray-fibers-idle-layer">
                    {衆管線.iter().copied().map(|管線| view! {
                        <Rime光纖靜態管線 管線=管線 內觀=內觀 幾何緩存=幾何緩存 />
                    }).collect_view()}
                </g>

                <g class="xray-nodes-layer">
                    {衆節點.iter().copied().map(|節點| view! {
                        <Rime機關面板 節點=節點 內觀=內觀 幾何緩存=幾何緩存 />
                    }).collect_view()}
                </g>

                <g class="xray-beams-active-layer">
                    {衆管線.into_iter().map(|管線| view! {
                        <Rime光纖純能光束 管線=管線 內觀=內觀 幾何緩存=幾何緩存 />
                    }).collect_view()}
                </g>
            </svg>
        </div>
    }
}

/// [內觀視圖] 底部微觀脈衝流水欄 (錄屏態下隱藏).
#[component]
pub fn Rime脈衝欄(內觀: 內觀總樞) -> impl IntoView {
    view! {
        <Show when=move || 內觀.開啓.get() && !內觀.錄屏模式.get()>
            <footer class="xray-pulse-ticker">
                <div class="xray-ticker-meta">
                    <span class="xray-brand-emblem">"◈"</span>
                    <span>{move || format!("微觀脈衝 #{}", 內觀.累計脈衝數.get())}</span>
                </div>
                <div class="xray-ticker-items">
                    {move || 內觀.近期脈衝.get().into_iter().take(6).map(|記錄| view! {
                        <span class="xray-ticker-badge">
                            <span class="ticker-role">{記錄.角色.符號()}</span>
                            <span class="ticker-sig">{記錄.信號名}</span>
                            <span class="ticker-val">{記錄.摘要}</span>
                        </span>
                    }).collect_view()}
                </div>
            </footer>
        </Show>
    }
}

#[cfg(test)]
mod 測試 {
    use super::*;
    use crate::introspect::合詠階段;

    #[test]
    fn 測試自動分層排版與光纖幾何無重疊() {
        let 衆節點 = vec![
            機關節點 {
                紐: 節點紐(0),
                標題: "方案樞 · 輸入方案",
                態勢: 節點態勢::實裝 {
                    模塊: "gear::theory",
                    主角色: 響應角色::源,
                },
            },
            機關節點 {
                紐: 節點紐(1),
                標題: "觸鍵源 · 輸入事件",
                態勢: 節點態勢::實裝 {
                    模塊: "gear::input",
                    主角色: 響應角色::源,
                },
            },
            機關節點 {
                紐: 節點紐(2),
                標題: "模式闕 · 工作模式",
                態勢: 節點態勢::實裝 {
                    模塊: "gear::mode",
                    主角色: 響應角色::源,
                },
            },
            機關節點 {
                紐: 節點紐(3),
                標題: "盤面陣 · 鍵盤佈局",
                態勢: 節點態勢::實裝 {
                    模塊: "gear::layout",
                    主角色: 響應角色::流,
                },
            },
            機關節點 {
                紐: 節點紐(4),
                標題: "作業卷 · 習題進度",
                態勢: 節點態勢::實裝 {
                    模塊: "gear::assignment",
                    主角色: 響應角色::澤,
                },
            },
            機關節點 {
                紐: 節點紐(5),
                標題: "並擊池 · 並擊機關",
                態勢: 節點態勢::實裝 {
                    模塊: "gear::chord",
                    主角色: 響應角色::澤,
                },
            },
            機關節點 {
                紐: 節點紐(6),
                標題: "連擊機 · 連擊機關",
                態勢: 節點態勢::實裝 {
                    模塊: "gear::key_press",
                    主角色: 響應角色::澤,
                },
            },
            機關節點 {
                紐: 節點紐(7),
                標題: "字幕軸 · 字幕機關",
                態勢: 節點態勢::實裝 {
                    模塊: "gear::caption",
                    主角色: 響應角色::流,
                },
            },
            機關節點 {
                紐: 節點紐(8),
                標題: "走紙屏 · 打字機視圖",
                態勢: 節點態勢::實裝 {
                    模塊: "view::*",
                    主角色: 響應角色::閘,
                },
            },
            機關節點 {
                紐: 節點紐(9),
                標題: "韻書探幽",
                態勢: 節點態勢::待鑄 {
                    設想職責: "韻書檢索",
                },
            },
            機關節點 {
                紐: 節點紐(10),
                標題: "DAG 構詞",
                態勢: 節點態勢::待鑄 {
                    設想職責: "切分構詞",
                },
            },
            機關節點 {
                紐: 節點紐(11),
                標題: "機關排序",
                態勢: 節點態勢::待鑄 {
                    設想職責: "候選排序",
                },
            },
        ];

        let 造線 = |編號, 源, 目標, 角色, 分組| 信號管線 {
            紐: 管線紐(編號),
            信號名: "測試信號",
            源節點: 節點紐(源),
            目標節點: 節點紐(目標),
            角色,
            分組,
            階段: 合詠階段::無,
        };

        let 衆管線 = vec![
            造線(0, 1, 5, 響應角色::掣, 九脈::觸鍵消息脈),
            造線(1, 1, 5, 響應角色::掣, 九脈::觸鍵消息脈),
            造線(2, 1, 6, 響應角色::掣, 九脈::觸鍵消息脈),
            造線(3, 1, 2, 響應角色::掣, 九脈::觸鍵消息脈),
            造線(4, 8, 1, 響應角色::掣, 九脈::觸鍵消息脈),
            造線(5, 0, 4, 響應角色::源, 九脈::方案法度脈),
            造線(6, 0, 3, 響應角色::流, 九脈::方案法度脈),
            造線(7, 0, 5, 響應角色::流, 九脈::方案法度脈),
            造線(8, 0, 6, 響應角色::流, 九脈::方案法度脈),
            造線(9, 0, 7, 響應角色::流, 九脈::方案法度脈),
            造線(10, 1, 3, 響應角色::澤, 九脈::盤面經緯脈),
            造線(11, 3, 8, 響應角色::閘, 九脈::盤面經緯脈),
            造線(12, 3, 6, 響應角色::澤, 九脈::盤面經緯脈),
            造線(13, 3, 8, 響應角色::閘, 九脈::盤面經緯脈),
            造線(14, 6, 3, 響應角色::掣, 九脈::盤面經緯脈),
            造線(15, 5, 8, 響應角色::源, 九脈::並擊和絃脈),
            造線(16, 5, 8, 響應角色::澤, 九脈::並擊和絃脈),
            造線(17, 5, 8, 響應角色::流, 九脈::並擊和絃脈),
            造線(18, 6, 8, 響應角色::流, 九脈::連擊字根脈),
            造線(19, 6, 8, 響應角色::流, 九脈::連擊字根脈),
            造線(20, 4, 7, 響應角色::澤, 九脈::習題碼表脈),
            造線(21, 4, 7, 響應角色::澤, 九脈::習題碼表脈),
            造線(22, 4, 5, 響應角色::流, 九脈::習題碼表脈),
            造線(23, 4, 6, 響應角色::流, 九脈::習題碼表脈),
            造線(24, 5, 8, 響應角色::閘, 九脈::反查指路脈),
            造線(25, 6, 8, 響應角色::閘, 九脈::反查指路脈),
            造線(26, 5, 4, 響應角色::掣, 九脈::批閱步進脈),
            造線(27, 6, 4, 響應角色::掣, 九脈::批閱步進脈),
            造線(28, 4, 7, 響應角色::源, 九脈::批閱步進脈),
            造線(29, 7, 8, 響應角色::閘, 九脈::走紙回顯脈),
            造線(30, 2, 8, 響應角色::閘, 九脈::走紙回顯脈),
            造線(31, 2, 4, 響應角色::源, 九脈::走紙回顯脈),
            造線(32, 5, 9, 響應角色::流, 九脈::待鑄藍圖脈),
            造線(33, 6, 9, 響應角色::流, 九脈::待鑄藍圖脈),
            造線(34, 9, 10, 響應角色::澤, 九脈::待鑄藍圖脈),
            造線(35, 10, 11, 響應角色::流, 九脈::待鑄藍圖脈),
            造線(36, 11, 8, 響應角色::閘, 九脈::待鑄藍圖脈),
        ];

        let 雙態 = 雙態視圖幾何::構建(&衆節點, &衆管線);
        assert_eq!(雙態.列標題.len(), 5);
        assert_eq!(雙態.節點座標[0].列, Some(0));
        assert_eq!(雙態.節點座標[1].列, Some(0));
        assert_eq!(雙態.節點座標[2].列, Some(0));
        assert_eq!(雙態.節點座標[3].列, Some(1));
        assert_eq!(雙態.節點座標[4].列, Some(1));
        assert_eq!(雙態.節點座標[5].列, Some(2));
        assert_eq!(雙態.節點座標[6].列, Some(2));
        assert_eq!(雙態.節點座標[7].列, Some(3));
        assert_eq!(雙態.節點座標[8].列, Some(4));

        // 校驗任意兩枚機關面板外框絕不交疊
        for 甲 in 0..衆節點.len() {
            for 乙 in (甲 + 1)..衆節點.len() {
                let dx = (雙態.節點座標[甲].x - 雙態.節點座標[乙].x).abs();
                let dy = (雙態.節點座標[甲].y - 雙態.節點座標[乙].y).abs();
                assert!(
                    dx >= 面板寬度 + 12.0 || dy >= 面板高度 + 12.0,
                    "面板 #{甲} 與 #{乙} 發生重疊: ({:.1}, {:.1}) vs ({:.1}, {:.1})",
                    雙態.節點座標[甲].x,
                    雙態.節點座標[甲].y,
                    雙態.節點座標[乙].x,
                    雙態.節點座標[乙].y,
                );
            }
        }

        for 編號 in 0..32 {
            assert!(雙態.查詢管線幾何(管線紐(編號), false).is_some());
        }
        for 編號 in 32..37 {
            assert!(雙態.查詢管線幾何(管線紐(編號), false).is_none());
            assert!(雙態.查詢管線幾何(管線紐(編號), true).is_some());
        }
    }
}
