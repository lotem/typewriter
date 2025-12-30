use super::練習題;
use crate::gear::caption::{
    字幕格式::{段落, 詞句},
    字幕步進::逐字,
};

pub const 字母鍵盤練習題: &[練習題] = &[
    練習題 {
        標題: "字母表",
        編碼: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        字幕: 詞句("ABCD EFGH IJKL MNOPQ RSTU VWXYZ"),
    },
    練習題 {
        標題: "QWERTY 鍵盤",
        編碼: "QWERTYUIOPASDFGHJKLZXCVBNM",
        字幕: 詞句(
            "QWERT YUIOP \
             ASDFG HJKL \
             ZXCVB NM",
        ),
    },
    練習題 {
        標題: "洋文金句",
        編碼: "hello world
the quick brown fox jumps over the lazy dog
black sheep wall
food for thought
operation cwal
power overwhelming
show me the money
something for nothing
there is no cow level",
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
];
