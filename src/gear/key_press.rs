use leptos::logging::log;
use leptos::prelude::*;

use crate::action::{ 动作给一参数, 动作 };
use crate::gear::{ assignment::作业模式输出信号, theory::输入方案输出信号 };
use crate::key_code::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct 连击状态 {
    pub 键码: KeyCode,
    pub 连击次数: usize,
}

impl Default for 连击状态 {
    fn default() -> Self {
        Self {
            键码: KeyCode::No,
            连击次数: 0,
        }
    }
}

impl 连击状态 {
    pub fn 击发(&mut self, 键码: KeyCode) {
        if 键码 == self.键码 {
            self.连击次数 += 1;
        } else {
            self.键码 = 键码;
            self.连击次数 = 1;
        }
    }
}

pub type 清空动作 = impl 动作;
pub type 回退动作 = impl 动作;
pub type 击键动作 = impl 动作给一参数<KeyCode>;

#[derive(Clone)]
pub struct 连击模式输出信号 {
    pub 连击状态变更: WriteSignal<连击状态>,
    pub 连击输入码: ReadSignal<Vec<String>>,
    pub 实况字根码: Signal<String>,
    pub 连击比对成功: Memo<bool>,
    pub 清空连击输入码: 清空动作,
    pub 回退连击输入码: 回退动作,
    pub 编辑连击输入码: 击键动作,
}

#[define_opaque(清空动作, 回退动作, 击键动作)]
pub fn 连击机关(方案: &输入方案输出信号, 作业: &作业模式输出信号) -> 连击模式输出信号 {
    let 方案 = 方案.方案定义;
    let 目标输入码片段 = 作业.目标输入码片段;

    let (连击状态流, 连击状态变更) = signal(连击状态::default());

    let 实况字根码 = Signal::derive(move || 方案.read().写成字根码(连击状态流.read().键码));
    let 反查所得字根码 = move || {
        目标输入码片段
            .read()
            .as_ref()
            .and_then(|对照码| 对照码.反查字根码(&方案.read().转写))
    };
    let 连击比对成功 = Memo::new(move |_|
        反查所得字根码().is_some_and(|查得| 查得 == 实况字根码())
    );

    let (连击输入码, 更新连击输入码) = signal(Vec::<String>::new());

    let 清空连击输入码 = move || {
        更新连击输入码.write().clear();
    };

    let 回退连击输入码 = move || {
        更新连击输入码.write().pop();
    };

    let 编辑连击输入码 = move |键码: KeyCode| {
        let 自由输入 = 目标输入码片段.read().is_none();
        let 击键正确 = 连击比对成功();
        if 自由输入 || 击键正确 {
            match 键码 {
                KeyCode::Space => {
                    let 空格 = 方案.read().写成字根码(KeyCode::Space);
                    更新连击输入码(vec![空格]);
                }
                键码 => {
                    let 字根码 = 方案.read().写成字根码(键码);
                    if !字根码.is_empty() {
                        log!("更新连击输入码 {字根码}");
                        let 空格 = 方案.read().写成字根码(KeyCode::Space);
                        if *连击输入码.read() == [空格] {
                            更新连击输入码(vec![字根码]);
                        } else {
                            更新连击输入码.write().push(字根码);
                        }
                    }
                }
            }
        }
    };

    连击模式输出信号 {
        连击状态变更,
        连击输入码,
        实况字根码,
        连击比对成功,
        清空连击输入码,
        回退连击输入码,
        编辑连击输入码,
    }
}
