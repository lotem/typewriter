mod alphabet;
mod ancient_chinese;
mod cantonese;
mod combo_pinyin;
mod double_pinyin_fly;

use alphabet::字母键盘练习题;
use ancient_chinese::{
    上古汉语练习题,
    早期中古汉语练习题,
    晚期中古汉语练习题,
    现代汉语练习题,
    近古汉语练习题,
};
use cantonese::{ 宫保粤拼练习题, 粤语练习题 };
use combo_pinyin::宫保拼音练习题;
use double_pinyin_fly::小鹤双拼练习题;

use crate::gear::{ caption::字幕格式, theory::方案选项 };

pub struct 练习题<'a> {
    pub 标题: &'a str,
    pub 编码: &'a str,
    pub 字幕: 字幕格式<'a>,
}

const 各方案练习题组: &[(方案选项, &[练习题])] = &[
    (方案选项::拉丁字母, 字母键盘练习题),
    (方案选项::宫保拼音, 宫保拼音练习题),
    (方案选项::上古汉语, 上古汉语练习题),
    (方案选项::早期中古汉语, 早期中古汉语练习题),
    (方案选项::晚期中古汉语, 晚期中古汉语练习题),
    (方案选项::近古汉语, 近古汉语练习题),
    (方案选项::现代汉语, 现代汉语练习题),
    (方案选项::粤语, 粤语练习题),
    (方案选项::宫保粤拼, 宫保粤拼练习题),
    (方案选项::小鹤双拼, 小鹤双拼练习题),
];

impl 方案选项 {
    pub fn 配套练习题(&self) -> Option<&'static [练习题<'static>]> {
        各方案练习题组.iter().find_map(|&(方案, 练习题)| {
            if 方案 == *self { Some(练习题) } else { None }
        })
    }
}
