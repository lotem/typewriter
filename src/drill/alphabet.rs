use super::練習題;

pub const 字母鍵盤練習題: &[練習題] = &[
    練習題::詞句(
        "字母表",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "ABCD EFGH IJKL MNOPQ RSTU VWXYZ",
    ),
    練習題::詞句(
        "QWERTY 鍵盤",
        "QWERTYUIOPASDFGHJKLZXCVBNM",
        "QWERT YUIOP ASDFG HJKL ZXCVB NM",
    ),
    練習題::直打(
        "洋文金句",
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
];
