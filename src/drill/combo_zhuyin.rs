use super::{
    練習題,
    題目來源::{內建, 求取},
};
use crate::gear::{assignment::碼表定義::並擊, caption::字幕格式::詞句};

pub const 宮保注音練習題: &[練習題] = &[
    練習題 {
        標題: "聲母",
        題目: 內建 {
            編碼: 並擊(
                "bu pa ma fan dong tai neng li \
                 gong kai hua jin qu xin \
                 shou ru zhi chu si zi ci",
            ),
            字幕: 詞句("不怕麻煩 動態能力 公開化 進取心 收入支出 四字詞"),
        },
    },
    練習題 {
        標題: "緣·驚·愧",
        題目: 內建 {
            編碼: 並擊(
                "you guo yi mian zhi yuan \
                 dang shi wei zhi zhen jing \
                 zhi jin zi kui fu ru",
            ),
            字幕: 詞句("有過一面之緣 當時爲之震驚 至今自愧弗如"),
        },
    },
    練習題 {
        標題: "東風破早梅",
        題目: 內建 {
            編碼: 並擊(
                "dong feng po zao mei \
                 xiang nuan yi zhi kai \
                 bing xue wu ren jian \
                 chun cong tian shang lai",
            ),
            字幕: 詞句("東風破早梅 向暖一枝開 冰雪無人見 春從天上來"),
        },
    },
    練習題 {
        標題: "東風破·注音緣",
        題目: 求取 {
            網址: "/typewriter/static/drill/dongfengpo-zhuyinyuan.txt",
        },
    },
];
