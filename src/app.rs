use leptos::prelude::*;

use crate::definition::{ 击键方式, 键组 };
use crate::engine::{ 微观引擎, 微观引擎输出信号 };
use crate::gear::{
    assignment::{ 作业模式输出信号, 作业 },
    caption::{ 字幕机关输出信号, 字幕表示 },
    chord::{ 并击模式输出信号, 并击状态 },
    key_press::连击模式输出信号,
    layout::{ 功能键::{ 制表键, 回车键, 退出键, 退格键 }, 键盘配列输出信号 },
    mode::{ 工作模式, 工作模式输出信号 },
    theory::输入方案输出信号,
};
use crate::key_code::KeyCode;
use crate::view::{
    caption::Rime字幕屏,
    exercise_menu::Rime练习题选单,
    input_code::{ Rime反查输入栏, Rime编码回显区, Rime编码栏, 回显区布局, 编码栏显示选项 },
    keyboard::{ Rime键图, Rime键盘图, 键面动态着色法 },
    layout_menu::Rime配列选单,
    status_bar::Rime状态栏,
    theory_menu::Rime方案选单,
};

#[derive(Clone, Copy)]
struct 并击对标动态 {
    目标并击: Signal<Option<键组>>,
    实况并击: Signal<并击状态>,
}

impl 键面动态着色法 for 并击对标动态 {
    fn 键位提示(&self, 键: KeyCode) -> bool {
        self.目标并击
            .read()
            .as_ref()
            .is_some_and(|并击| 并击.0.contains(&键))
    }

    fn 是否落键(&self, 键: KeyCode) -> bool {
        self.实况并击.read().实时击键.0.contains(&键)
    }

    fn 是否击中(&self, 键: KeyCode) -> bool {
        self.实况并击.read().累计击键.0.contains(&键)
    }
}

#[derive(Clone, Copy)]
struct 功能键开关状态 {
    现行工作模式: ReadSignal<工作模式>,
}

impl 键面动态着色法 for 功能键开关状态 {
    fn 键位提示(&self, _键: KeyCode) -> bool {
        false
    }

    fn 是否落键(&self, 键: KeyCode) -> bool {
        match 键 {
            KeyCode::Enter => self.现行工作模式.get() == 工作模式::输入反查码,
            KeyCode::Escape => self.现行工作模式.get() == 工作模式::选取练习题,
            KeyCode::Grave => self.现行工作模式.get() == 工作模式::选择输入方案,
            _ => false,
        }
    }

    fn 是否击中(&self, _键: KeyCode) -> bool {
        false
    }
}

#[component]
pub fn Rime打字机应用() -> impl IntoView {
    let 微观引擎输出信号 { 方案, 模式, 键盘配列, 作业, 字幕, 连击, 并击 } = 微观引擎();
    let 输入方案输出信号 { 当前方案, 选用方案, 方案定义, 指法, .. } = 方案;
    let 键盘配列输出信号 { 已选配列, 选用配列 } = 键盘配列;
    let 工作模式输出信号 {
        现行工作模式,
        开启反查输入,
        开启练习题选单,
        关闭输入栏,
        开启方案选单,
        开启配列选单,
        ..
    } = 模式;
    let 作业模式输出信号 { 当前作业, 布置作业, 目标输入码片段, .. } = 作业;
    let 字幕机关输出信号 { 段落表示, .. } = 字幕;
    let 连击模式输出信号 { 连击输入码, 实况字根码, .. } = 连击;
    let 并击模式输出信号 {
        并击状态流,
        反查键位,
        反查所得并击码,
        实际并击码,
        并击所得拼音,
        并击完成,
        并击成功,
        ..
    } = 并击;

    let 是否显示光标 = Signal::derive(move || matches!(指法(), 击键方式::连击));
    let 有无输入码 = Signal::derive(move || {
        match 指法() {
            击键方式::连击 => !实况字根码.read().is_empty(),
            击键方式::并击 => !实际并击码.read().is_empty(),
        }
    });
    let 显示选项 = Signal::derive(move || {
        if 反查键位.read().is_some() {
            编码栏显示选项::显示反查
        } else if 有无输入码() {
            编码栏显示选项::显示实况
        } else {
            编码栏显示选项::无显示
        }
    });
    let 完成一词 = move || {
        段落表示.read()
            .as_ref()
            .is_some_and(|字幕表示 { 指标文字, .. }| ["", " "].contains(&指标文字.as_str()))
    };
    let 输入正确 = Signal::derive(move || {
        match 指法() {
            击键方式::连击 => 完成一词(),
            击键方式::并击 => 并击完成() && 并击成功(),
        }
    });
    let 点击编码栏动作 = move || {
        if 现行工作模式() == 工作模式::录入 {
            if 当前作业.read().是否练习题() {
                开启练习题选单();
            } else {
                开启反查输入();
            }
        }
    };
    let 编码回显区布局 = Signal::derive(move || {
        match 指法() {
            击键方式::连击 => 回显区布局::单栏,
            击键方式::并击 => 回显区布局::左右对照,
        }
    });
    let 回显输入码 = Signal::derive(move || {
        match 指法() {
            击键方式::连击 => {
                let 输入码 = 连击输入码.read().join("");
                match 输入码.as_str() {
                    "" | "␣" => 输入码,
                    _ => 输入码 + "‸",
                }
            }
            击键方式::并击 => 反查所得并击码().unwrap_or_else(实际并击码),
        }
    });
    let 回显转写码 = Signal::derive(move || {
        match 指法() {
            击键方式::连击 => None,
            击键方式::并击 => {
                目标输入码片段()
                    .and_then(|输入码| 输入码.转写码原文)
                    .or_else(并击所得拼音)
                    // 加尖括弧表示拉丁文转写
                    .map(|转写| format!("⟨{转写}⟩"))
            }
        }
    });
    let 反查码 = Signal::derive(move || {
        当前作业.read()
            .自定义反查码.clone()
            .or_else(|| 当前作业.read().目标输入码().map(str::to_owned))
    });
    let 反查码变更动作 = move |反查码: String| {
        布置作业(作业::自定义(当前方案(), 反查码));
    };
    let 当前题号 = Signal::derive(move || 当前作业.read().题号);
    let 选中题号动作 = move |题号| {
        布置作业(作业::练习题(当前方案(), 题号));
        关闭输入栏();
    };
    let 选中方案动作 = move |选中项| {
        选用方案(选中项);
        关闭输入栏();
    };
    let 选用配列动作 = move |选中项| {
        选用配列(选中项);
        关闭输入栏();
    };
    let 方案配套练习题 = Signal::derive(move || 当前方案.get().配套练习题().unwrap_or(&[]));
    let 方案指定布局 = Signal::derive(move || *方案定义.read().布局);
    let 方案指定盘面 = Signal::derive(move || 方案指定布局().默认盘面);

    let 标注功能键 = |功能键| Signal::derive(move || 功能键);

    let 并击动态 = 并击对标动态 {
        目标并击: 反查键位,
        实况并击: 并击状态流.into(),
    };

    let 开关状态 = 功能键开关状态 { 现行工作模式 };

    view! {
        <Rime字幕屏 是否显示光标={是否显示光标} 按进度显示字幕={字幕.段落表示}/>
        <div class="echo-bar">
            <div title="重新录入／选练习题">
                <Rime键图 键={退出键.键码} 标注法={标注功能键(退出键)} 着色法={开关状态}/>
            </div>
            <div title="下一题">
                <Rime键图 键={制表键.键码} 标注法={标注功能键(制表键)} 着色法={并击动态}/>
            </div>
            <Rime编码栏
                显示选项={显示选项}
                输入正确={输入正确}
                点击动作={点击编码栏动作}
                关闭输入栏={关闭输入栏}
            >
            {
                move || match 现行工作模式() {
                    工作模式::录入 => view! {
                        <Rime编码回显区 布局={编码回显区布局} 输入码={回显输入码} 转写码={回显转写码}/>
                    }.into_any(),
                    工作模式::输入反查码 => view! {
                        <Rime反查输入栏
                            反查码={反查码}
                            示例输入={Signal::derive(|| String::from("qing shu ru pin yin"))}
                            反查码变更={反查码变更动作}
                        />
                    }.into_any(),
                    工作模式::选取练习题 => view! {
                        <Rime练习题选单
                            预设练习题={方案配套练习题}
                            当前题号={当前题号}
                            选择题号={选中题号动作}
                        />
                    }.into_any(),
                    工作模式::选择输入方案 => view! {
                        <Rime方案选单
                            当前方案={当前方案}
                            选中方案={选中方案动作}
                        />
                    }.into_any(),
                    工作模式::选择配列 => view! {
                        <Rime配列选单
                            已选配列={已选配列}
                            选用配列={选用配列动作}
                        />
                    }.into_any(),
                }
            }
            </Rime编码栏>
            <div title="输入拼音反查键位">
                <Rime键图 键={回车键.键码} 标注法={标注功能键(回车键)} 着色法={开关状态}/>
            </div>
            <div title="删除／回退一字">
                <Rime键图 键={退格键.键码} 标注法={标注功能键(退格键)} 着色法={并击动态}/>
            </div>
        </div>
        <Rime键盘图 键盘布局={方案指定布局} 目标盘面={方案指定盘面} 键盘配列={已选配列} 着色法={并击动态}/>

        <Rime状态栏
            当前方案={当前方案}
            已选配列={已选配列}
            点击方案={move || 开启方案选单()}
            点击配列={move || 开启配列选单()}
        />
    }
}
