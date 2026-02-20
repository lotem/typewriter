use leptos::prelude::*;
use leptos_router::{
    hooks::{use_navigate, use_params, use_query},
    params::Params,
    NavigateOptions,
};

use crate::{
    action::動作給一參數,
    gear::{layout::配列, theory::方案選項},
};

// 路徑參數: 處理 /:theory
#[derive(Params, PartialEq, Clone, Debug)]
pub struct AppParams {
    pub theory: String,
}

// 查詢參數: ?drill=...&layout=...
#[derive(Params, PartialEq, Clone, Debug)]
pub struct AppQuery {
    pub drill: Option<String>,
    pub layout: Option<String>,
}

impl 方案選項 {
    // 生成 URL
    fn slug(&self) -> &str {
        match self {
            方案選項::宮保拼音 => "combo_pinyin",
            方案選項::拉丁字母 => "alphabet",
            方案選項::上古漢語 => "old_chinese",
            方案選項::早期中古漢語 => "early_middle_chinese",
            方案選項::晚期中古漢語 => "late_middle_chinese",
            方案選項::近古漢語 => "old_mandarin",
            方案選項::現代漢語 => "modern_chinese",
            方案選項::粵語 => "cantonese",
            方案選項::宮保粵拼 => "combo_jyutping",
            方案選項::注音 => "zhuyin",
            方案選項::動態能力注音 => "detenele",
            方案選項::宮保注音 => "combo_zhuyin",
        }
    }
    // 解析 URL
    fn from_slug(s: &str) -> Result<Self, ()> {
        match s {
            "combo_pinyin" => Ok(方案選項::宮保拼音),
            "alphabet" => Ok(方案選項::拉丁字母),
            "old_chinese" => Ok(方案選項::上古漢語),
            "early_middle_chinese" => Ok(方案選項::早期中古漢語),
            "late_middle_chinese" => Ok(方案選項::晚期中古漢語),
            "old_mandarin" => Ok(方案選項::近古漢語),
            "modern_chinese" => Ok(方案選項::現代漢語),
            "cantonese" => Ok(方案選項::粵語),
            "combo_jyutping" => Ok(方案選項::宮保粵拼),
            "zhuyin" => Ok(方案選項::注音),
            "detenele" => Ok(方案選項::動態能力注音),
            "combo_zhuyin" => Ok(方案選項::宮保注音),
            _ => Err(()), // 解析失敗
        }
    }
}

impl 配列 {
    // 生成 URL
    fn slug(&self) -> &str {
        match self {
            配列::主鍵盤區 => "qwerty",
            配列::字母鍵盤 => "alphabet",
            配列::正交直列 => "ortho",
            配列::直列分體 => "ortho_split",
            配列::正交直列帶數字行 => "ortho_with_number_row",
            配列::直列分體帶數字行 => "ortho_split_with_number_row",
        }
    }
    // 解析 URL
    fn from_slug(s: &str) -> Result<Self, ()> {
        match s {
            "qwerty" => Ok(配列::主鍵盤區),
            "alphabet" => Ok(配列::字母鍵盤),
            "ortho" => Ok(配列::正交直列),
            "ortho_split" => Ok(配列::直列分體),
            "ortho_with_number_row" => Ok(配列::正交直列帶數字行),
            "ortho_split_with_number_row" => Ok(配列::直列分體帶數字行),
            _ => Err(()), // 解析失敗
        }
    }
}

pub type 選用方案動作 = impl 動作給一參數<方案選項>;
pub type 選用練習題動作 = impl 動作給一參數<Option<usize>>;
pub type 選用配列動作 = impl 動作給一參數<Option<配列>>;

// 返回值類型, 包含所有狀態和設置器
pub struct AppState {
    pub theory: Signal<方案選項>,
    pub drill: Signal<Option<usize>>,
    pub layout: Signal<Option<配列>>,
    pub set_theory: 選用方案動作,
    pub set_drill: 選用練習題動作,
    pub set_layout: 選用配列動作,
}

#[define_opaque(選用方案動作, 選用練習題動作, 選用配列動作)]
pub fn use_app_state() -> AppState {
    let params = use_params::<AppParams>();
    let query = use_query::<AppQuery>();

    // 從路徑獲取 theory, 默認 [宮保拼音]
    let theory = Signal::derive(move || {
        params
            .read()
            .as_ref()
            .ok()
            .map(|x| x.theory.clone())
            .and_then(|s| 方案選項::from_slug(&s).ok())
            .unwrap_or_default()
    });

    // 從 query 獲取 drill, 冇 = None
    let drill = Signal::derive(move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|x| x.drill.as_ref().and_then(|x| x.parse::<usize>().ok()))
    });

    // 從 query 獲取 layout, 冇 = None
    let layout = Signal::derive(move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|x| x.layout.clone())
            .and_then(|s| 配列::from_slug(&s).ok())
    });

    let navigate =
        move |new_theory: 方案選項, new_drill: Option<usize>, new_layout: Option<配列>| {
            let query_items = [
                new_drill.map(|drill| format!("drill={}", drill)),
                new_layout.map(|layout| format!("layout={}", layout.slug())),
            ];
            let query_str = query_items
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join("&");
            let query_suffix = if query_str.is_empty() {
                String::new()
            } else {
                format!("?{}", query_str)
            };
            let url = format!("/typewriter/{}{}", new_theory.slug(), query_suffix);
            let navigate = use_navigate();
            navigate(
                &url,
                NavigateOptions {
                    resolve: false,
                    replace: true, // 替換歷史記錄, 後退鍵管用
                    ..Default::default()
                },
            );
        };

    let set_theory = move |new_val: 方案選項| {
        // 清空 drill 和 layout 參數, 使用方案指定的默認佈局和該方案的 0 號練習題
        navigate(new_val, None, None);
    };

    let set_drill = move |new_val: Option<usize>| {
        navigate(theory.get_untracked(), new_val, layout.get_untracked());
    };

    let set_layout = move |new_val: Option<配列>| {
        navigate(theory.get_untracked(), drill.get_untracked(), new_val);
    };

    AppState {
        theory,
        drill,
        layout,
        set_theory,
        set_drill,
        set_layout,
    }
}
