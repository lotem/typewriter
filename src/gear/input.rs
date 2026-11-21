use std::collections::BTreeMap;

use leptos::{ev, leptos_dom::helpers::window_event_listener, logging::log, prelude::*};
use leptos_use::use_window_focus;

use crate::action::{動作, 動作給一參數, 動作給一參數得一結果};
use crate::key_code::{網頁鍵值轉換, KeyCode};

/// 按需生成的細粒度觸鍵狀態映射表 ([統一按鍵綁定] 的上游電平雛形):
/// 僅爲已登記訂閱的鍵碼分配獨立 `RwSignal<bool>` 槽位; 未訂閱鍵位的落/抬不產生任何信號發射.
#[derive(Clone, Copy)]
pub struct 按鍵狀態表 {
    槽表: StoredValue<BTreeMap<KeyCode, RwSignal<bool>>>,
}

impl 按鍵狀態表 {
    pub fn new() -> Self {
        Self {
            槽表: StoredValue::new(BTreeMap::new()),
        }
    }

    /// 按需訂閱單顆實體鍵的實時落鍵狀態.
    pub fn 訂閱落鍵態(&self, 鍵碼: KeyCode) -> ReadSignal<bool> {
        self.槽表
            .write_value()
            .entry(鍵碼)
            .or_insert_with(|| RwSignal::new(false))
            .read_only()
    }

    /// 按需訂閱一組同義實體鍵 (如左右 Shift) 的聚合落鍵狀態, 經 `Memo` 等值截流去重.
    pub fn 訂閱任一落鍵態(&self, 衆鍵碼: &[KeyCode]) -> Memo<bool> {
        let 衆鍵態: Vec<ReadSignal<bool>> = 衆鍵碼
            .iter()
            .copied()
            .map(|鍵碼| self.訂閱落鍵態(鍵碼))
            .collect();
        Memo::new(move |_| 衆鍵態.iter().any(|鍵態| 鍵態.get()))
    }

    pub fn 落鍵(&self, 鍵碼: KeyCode) {
        let 目標槽 = self.槽表.with_value(|表| 表.get(&鍵碼).copied());
        if let Some(槽) = 目標槽 {
            if !槽.get_untracked() {
                槽.set(true);
            }
        }
    }

    pub fn 抬鍵(&self, 鍵碼: KeyCode) {
        let 目標槽 = self.槽表.with_value(|表| 表.get(&鍵碼).copied());
        if let Some(槽) = 目標槽 {
            if 槽.get_untracked() {
                槽.set(false);
            }
        }
    }

    /// 根據瀏覽器鍵盤事件自帶修飾鍵旗標自癒遺失的 `keyup` 殘留狀態 (不干擾雙側同修飾鍵獨立釋放).
    pub fn 校準修飾鍵(&self, 當前鍵碼: KeyCode, 檔位: &檔位) {
        if 當前鍵碼.是否修飾鍵() {
            return;
        }
        let 過期檢查 = [
            (!檔位.shift, KeyCode::LShift),
            (!檔位.shift, KeyCode::RShift),
            (!檔位.ctrl, KeyCode::LCtrl),
            (!檔位.ctrl, KeyCode::RCtrl),
            (!檔位.alt, KeyCode::LAlt),
            (!檔位.alt, KeyCode::RAlt),
            (!檔位.meta, KeyCode::LGui),
            (!檔位.meta, KeyCode::RGui),
        ];
        for (須清除, 鍵碼) in 過期檢查 {
            if 須清除 {
                self.抬鍵(鍵碼);
            }
        }
    }

    pub fn 清空(&self) {
        let 全部槽: Vec<RwSignal<bool>> = self.槽表.with_value(|表| 表.values().copied().collect());
        for 槽 in 全部槽 {
            if 槽.get_untracked() {
                槽.set(false);
            }
        }
    }
}

pub fn 焦點事件處理機關(失焦清理: impl 動作) {
    let 鍵盤輸入焦點源 = Selector::new(use_window_focus());
    Effect::new(move |_| {
        if 鍵盤輸入焦點源.selected(&false) {
            失焦清理();
        }
    });
}

#[allow(dead_code)]
#[derive(Default)]
pub struct 檔位 {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

pub struct 觸鍵消息 {
    pub 鍵碼: KeyCode,
    pub 檔位: 檔位,
}

pub fn 輸入事件處理機關(
    鍵態表: 按鍵狀態表,
    處理功能鍵: impl 動作給一參數得一結果<觸鍵消息, bool>,
    既然落鍵: impl 動作給一參數<KeyCode>,
    既然抬鍵: impl 動作給一參數<KeyCode>,
) {
    let keydown_handle = window_event_listener(ev::keydown, move |ev| {
        if ev.is_composing() {
            return;
        }
        log!("落鍵 key = {}, code = {}", &ev.key(), ev.code());
        let 鍵碼 = 網頁鍵值轉換(&ev.code());
        let 檔位 = 檔位 {
            shift: ev.shift_key(),
            ctrl: ev.ctrl_key(),
            alt: ev.alt_key(),
            meta: ev.meta_key(),
        };
        鍵態表.校準修飾鍵(鍵碼, &檔位);
        鍵態表.落鍵(鍵碼);
        if 處理功能鍵(觸鍵消息 { 鍵碼, 檔位 }) {
            ev.prevent_default();
            return;
        }
        if 鍵碼 != KeyCode::No && !鍵碼.是否修飾鍵() {
            既然落鍵(鍵碼);
        }
    });

    let keyup_handle = window_event_listener(ev::keyup, move |ev| {
        if ev.is_composing() {
            return;
        }
        log!("抬鍵 key = {}, code = {}", &ev.key(), &ev.code());
        let 鍵碼 = 網頁鍵值轉換(&ev.code());
        鍵態表.抬鍵(鍵碼);
        if 鍵碼 != KeyCode::No && !鍵碼.是否修飾鍵() {
            既然抬鍵(鍵碼);
        }
    });

    on_cleanup(move || {
        keydown_handle.remove();
        keyup_handle.remove();
    });
}
