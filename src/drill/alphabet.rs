use super::练习题;
use crate::gear::caption::{
    字幕格式::{段落, 词句},
    字幕步进::逐字,
};

pub const 字母键盘练习题: &[练习题] = &[
    练习题 {
        标题: "字母表",
        编码: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        字幕: 词句("ABCD EFGH IJKL MNOPQ RSTU VWXYZ"),
    },
    练习题 {
        标题: "QWERTY 鍵盤",
        编码: "QWERTYUIOPASDFGHJKLZXCVBNM",
        字幕: 词句(
            "QWERT YUIOP \
             ASDFG HJKL \
             ZXCVB NM",
        ),
    },
    练习题 {
        标题: "洋文金句",
        编码: "HELLO␣WORLD
               THE␣QUICK␣BROWN␣FOX␣JUMPS␣OVER␣THE␣LAZY␣DOG",
        字幕: 段落(
            逐字,
            "hello world
             the quick brown fox jumps over the lazy dog",
        ),
    },
];
