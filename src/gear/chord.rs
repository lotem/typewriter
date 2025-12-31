use leptos::prelude::*;

use crate::action::动作;
use crate::definition::键组;
use crate::gear::{ assignment::作业模式输出信号, theory::输入方案输出信号 };
use crate::key_code::KeyCode;

pub struct 并击状态 {
    pub 实时击键: 键组,
    pub 累计击键: 键组,
}

impl 并击状态 {
    pub fn new() -> Self {
        并击状态 {
            实时击键: 键组::new(),
            累计击键: 键组::new(),
        }
    }

    pub fn 落键(&mut self, 键码: KeyCode) {
        if self.实时击键.0.is_empty() {
            self.并击开始();
        }
        self.实时击键.0.insert(键码);
        self.累计击键.0.insert(键码);
    }

    pub fn 抬键(&mut self, 键码: KeyCode) {
        self.实时击键.0.remove(&键码);
        if self.实时击键.0.is_empty() {
            self.并击完成();
        }
    }

    pub fn 重置(&mut self) {
        self.实时击键.0.clear();
        self.累计击键.0.clear();
    }

    pub fn 并击开始(&mut self) {
        self.重置();
    }

    pub fn 并击完成(&mut self) {}
}

pub type 并击重置动作 = impl 动作;

#[derive(Clone)]
pub struct 并击模式输出信号 {
    pub 并击状态流: ReadSignal<并击状态>,
    pub 并击状态变更: WriteSignal<并击状态>,
    pub 重置并击状态: 并击重置动作,
    pub 实际并击码: Memo<String>,
    pub 并击所得拼音: Signal<Option<String>>,
    pub 反查所得并击码: Memo<Option<String>>,
    pub 反查键位: Signal<Option<键组>>,
    pub 并击完成: Signal<bool>,
    pub 并击成功: Memo<bool>,
}

#[define_opaque(并击重置动作)]
pub fn 并击模式(方案: &输入方案输出信号, 作业: &作业模式输出信号) -> 并击模式输出信号 {
    let 方案定义 = 方案.方案定义;
    let 目标输入码片段 = 作业.目标输入码片段;

    let (并击状态流, 并击状态变更) = signal(并击状态::new());

    let 重置并击状态 = move || {
        并击状态变更.write().重置();
    };

    let 实际并击码 = Memo::new(move |_| 方案定义.read().写成字根码(&并击状态流.read().累计击键));
    let 并击所得拼音 = Signal::derive(move || 方案定义.read().转写.字根码转写为拼式(&实际并击码()));

    let 反查所得并击码 = Memo::new(move |_| {
        目标输入码片段
            .read()
            .as_ref()
            .and_then(|对照码| 对照码.反查字根码(&方案定义.read().转写))
    });
    let 反查键位 = Signal::derive(move || {
        反查所得并击码
            .read()
            .as_deref()
            .map(|并击码| 方案定义.read().读出键位(并击码))
    });

    let _并击开始 = Signal::derive(move || !并击状态流.read().实时击键.0.is_empty());

    let 并击完成 = Signal::derive(move || {
        并击状态流.read().实时击键.0.is_empty() && !实际并击码.read().is_empty()
    });

    let 并击成功 = Memo::new(move |_| {
        // 拼音一致即为成功，允许并击码不同
        目标输入码片段().is_some_and(|输入码| {
            输入码.字根码原文.is_some_and(|查得| 查得 == 实际并击码()) ||
                输入码.转写码原文.is_some_and(|查得|
                    并击所得拼音().is_some_and(|击得| 查得 == 击得)
                )
        }) ||
            // 拼音为非音节形式的声母、韵母，须比较并击码
            反查所得并击码().is_some_and(|查得| 查得 == 实际并击码())
    });

    并击模式输出信号 {
        并击状态流,
        并击状态变更,
        重置并击状态,
        实际并击码,
        并击所得拼音,
        反查所得并击码,
        反查键位,
        并击完成,
        并击成功,
    }
}
