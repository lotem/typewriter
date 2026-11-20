use super::練習題;

pub const 倉頡練習題: &[練習題] = &[
    練習題::逐鍵(
        "倉頡字母",
        "日月金木水火土竹戈十大中一弓人心手口尸廿山女田卜難",
        "日月金木水火土 竹戈十大中一弓 人心手口 尸廿山女田卜 難",
    ),
    練習題::求取("千字文", "/typewriter/static/drill/qianziwen.txt"),
    練習題::求取("鍵道·破執", "/typewriter/static/drill/jiandao-pozhi.txt"),
];
