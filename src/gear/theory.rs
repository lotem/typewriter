use lazy_static::lazy_static;
use leptos::prelude::*;

use crate::definition::{ 击键方式, 转写定义, 输入方案定义 };
use crate::gear::layout::拉丁字母键盘布局; // 默认键盘布局
use crate::theory::*; // 输入方案

#[derive(Clone, Copy, Default, PartialEq)]
pub enum 方案选项 {
    #[default]
    宫保拼音,
    拉丁字母,
    上古汉语,
    早期中古汉语,
    晚期中古汉语,
    近古汉语,
    现代汉语,
    粤语,
    宫保粤拼,
    小鹤双拼,
}

lazy_static! {
    pub static ref 方案列表: Vec<(方案选项, 输入方案定义<'static>)> = vec![
        (方案选项::宫保拼音, combo_pinyin::输入方案()),
        (方案选项::拉丁字母, alphabet::输入方案()),
        (方案选项::上古汉语, old_chinese::输入方案()),
        (方案选项::早期中古汉语, early_middle_chinese::输入方案()),
        (方案选项::晚期中古汉语, late_middle_chinese::输入方案()),
        (方案选项::近古汉语, old_mandarin::输入方案()),
        (方案选项::现代汉语, modern_chinese::输入方案()),
        (方案选项::粤语, cantonese::输入方案()),
        (方案选项::宫保粤拼, combo_jyutping::输入方案()),
        (方案选项::小鹤双拼, double_pinyin_fly::输入方案())
    ];
}

const 未定义方案: 输入方案定义<'static> = 输入方案定义 {
    名称: "未定义",
    布局: &拉丁字母键盘布局,
    指法: 击键方式::连击,
    键位映射: &[],
    转写: 转写定义 {
        编码预览: &[],
        键位提示: &[],
        输入棱镜: &[],
        词库棱镜: &[],
        拼式验证规则: &[],
    },
};

#[derive(Clone, Copy)]
pub struct 输入方案输出信号 {
    pub 当前方案: ReadSignal<方案选项>,
    pub 选用方案: WriteSignal<方案选项>,
    pub 方案定义: Signal<输入方案定义<'static>>,
    pub 指法: Signal<击键方式>,
}

pub fn 输入方案() -> 输入方案输出信号 {
    let (当前方案, 选用方案) = signal(方案选项::default());

    let 方案定义 = Signal::derive(move || {
        方案列表.iter()
            .find_map(|&(方案, 定义)| {
                if 方案 == 当前方案() { Some(定义) } else { None }
            })
            .unwrap_or(未定义方案)
    });

    let 指法 = Signal::derive(move || 方案定义.read().指法);

    输入方案输出信号 {
        当前方案,
        选用方案,
        方案定义,
        指法,
    }
}
