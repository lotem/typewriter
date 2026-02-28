use super::{練習題, 題目來源::內建};
use crate::gear::{
    assignment::碼表定義::逐鍵,
    caption::{
        字幕格式::{段落, 詞句},
        字幕步進::逐字,
    },
};

pub const 字母鍵盤練習題: &[練習題] = &[
    練習題 {
        標題: "字母表",
        題目: 內建 {
            編碼: 逐鍵("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            字幕: 詞句("ABCD EFGH IJKL MNOPQ RSTU VWXYZ"),
        },
    },
    練習題 {
        標題: "QWERTY 鍵盤",
        題目: 內建 {
            編碼: 逐鍵("QWERTYUIOPASDFGHJKLZXCVBNM"),
            字幕: 詞句(
                "QWERT YUIOP \
                 ASDFG HJKL \
                 ZXCVB NM",
            ),
        },
    },
    練習題 {
        標題: "洋文金句",
        題目: 內建 {
            編碼: 逐鍵(
                "hello world
the quick brown fox jumps over the lazy dog
black sheep wall
food for thought
operation cwal
power overwhelming
show me the money
something for nothing
there is no cow level",
            ),
            字幕: 段落(
                逐字,
                "hello world
the quick brown fox jumps over the lazy dog
black sheep wall
food for thought
operation cwal
power overwhelming
show me the money
something for nothing
there is no cow level",
            ),
        },
    },
];
