use super::練習題;
use crate::gear::{assignment::碼表定義::並擊, caption::字幕格式::詞句};

pub const 宮保注音練習題: &[練習題] = &[
    練習題 {
        標題: "聲母",
        編碼: 並擊(
            "bu pa ma fan dong tai neng li \
                    gong kai hua jin qu xin \
                    shou ru zhi chu si zi ci",
        ),
        字幕: 詞句("不怕麻煩 動態能力 公開化 進取心 收入支出 四字詞"),
    },
    練習題 {
        標題: "緣·驚·愧",
        編碼: 並擊(
            "you guo yi mian zhi yuan \
             dang shi wei zhi zhen jing \
             zhi jin zi kui fu ru",
        ),
        字幕: 詞句("有過一面之緣 當時爲之震驚 至今自愧弗如"),
    },
    練習題 {
        標題: "東風破早梅",
        編碼: 並擊(
            "dong feng po zao mei \
             xiang nuan yi zhi kai \
             bing xue wu ren jian \
             chun cong tian shang lai",
        ),
        字幕: 詞句("東風破早梅 向暖一枝開 冰雪無人見 春從天上來"),
    },
    練習題 {
        標題: "東風破·注音緣",
        編碼: 並擊(
            "dong feng po zhu yin yuan

[ㄙㄧ]=<xi> ri wu ban tu
da hui [ㄗㄩ]=<ju> qun ru
chu shi fu zhen jun
yuan fen ci zhong zhu

liang zai kai fa lu
pin yin chu shi bu
hu jian di yi ren
ling wo jing zhu

kan ta

hui hao shou [ㄙㄧㄝ]=<xie> zhu yin
fu hao chun shu
dang shi wei zhi zhen jing
zi kui fu ru

bian cheng gao chao zhong wen zao yi shuang [ㄘㄩㄢ]=<quan>
qi you bu hao yong zhi li
shi zhong bu bian re [ㄘㄧㄥ]=<qing> tou ru kai yuan lu
fa ming zhe shen wu

cheng [ㄙㄧㄣ]=<xin> xiang jun zhi jing
yi dai da zong shi",
        ),
        字幕: 詞句(
            "東風[破·]注音緣

昔日烏班圖
大會聚羣儒
初識佛振君
緣分此中駐

兩載開發路
拼音初試步
忽見第一人
令[我——]驚矚

看[他——]

揮毫手寫注音
符號純熟
當時爲之震驚
自愧弗如

編程高超中文造詣雙全
豈有不好用之理
始終不變熱情投入開源路
發明這神物

誠心向君致敬
一代大宗師",
        ),
    },
];
