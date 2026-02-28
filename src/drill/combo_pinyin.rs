use super::{
    練習題,
    題目來源::{內建, 求取},
};
use crate::gear::{
    assignment::碼表定義::並擊,
    caption::{
        字幕格式::{段落, 詞句},
        字幕步進::逐詞,
    },
};

pub const 宮保拼音練習題: &[練習題] = &[
    練習題 {
        標題: "中州韻輸入法引擎",
        題目: 內建 {
            編碼: 並擊("zhong zhou yun shu ru fa yin qing \
                        si xiang yong jian pan biao da ye xing"),
            字幕: 詞句("中州韻輸入法引擎 思想用鍵盤表達也行"),
        }
    },
    練習題{
        標題: "東風破早梅",
        題目: 內建 {
            編碼: 並擊("dong feng po zao mei \
                        xiang nuan yi zhi kai \
                        bing xue wu ren jian \
                        chun cong tian shang lai"),
            字幕: 詞句("東風破早梅 向暖一枝開 冰雪無人見 春從天上來"),
        },
    },
    練習題 {
        標題: "聲母韻母練習",
        題目: 內建 {
            編碼: 並擊("b p m f d t n l \
                        g k h j q x \
                        zh ch sh r z c s \
                        i u ü \
                        A=a ia ua o uo e ie üe er \
                        ai uai ei uei ao iao ou iou \
                        an ian uan üan en in uen ün \
                        ang iang uang eng ing ueng ong iong"),
            字幕: 段落(逐詞, "b p m f d t n l
g k h j q x
zh ch sh r z c s
i u ü
a ia ua o uo e ie üe er
ai uai ei uei ao iao ou iou
an ian uan üan en in uen ün
ang iang uang eng ing ueng ong iong"),
        },
    },
    練習題 {
        標題: "音節練習",
        題目: 內建 {
            編碼: 並擊("bu pu fu me \
                        de te le ne \
                        ge ke he \
                        zhi chi shi ri zi ci si \
                        er AE=<'a> A=<␣>"),
        字幕: 段落(逐詞, "bu pu fu me
de te le ne
ge ke he
zhi chi shi ri zi ci si
er 'a ␣"),
        },
    },
    練習題 {
        標題: "單字練習",
        題目: 內建 {
            編碼: 並擊("yi er san si wu liu qi ba jiu shi \
                        ren kou shou shang zhong xia ri yue shui huo shan shi tian tu \
                        mu he mi zhu dao gong che zhou qian hou zuo you er mu she ya \
                        tou zu zuo li zou chi cun yuan jiao fen \
                        da xiao duo shao jin liang ma niu yang mao pi \
                        niao chong yu zhua zi wei ba kai men guan chu ru lai qu \
                        fang xiang dong nan xi bei bai tian hei ye yun dian feng yu \
                        fu qin mu er nü ba ma"),
            字幕: 詞句("一二三四五 六七八九十 \
                        人口手 上中下 日月水火 山石田土 \
                        木禾米竹 刀弓車舟 前後左右 耳目舌牙 \
                        頭足坐立走 尺寸元角分 \
                        大小多少斤兩 馬牛羊毛皮 \
                        鳥蟲魚爪子尾巴 開門關出入來去 \
                        方向東南西北 白天黑夜雲電風雨 \
                        父親母兒女爸媽"),
        },
    },
    練習題 {
        標題: "綜合練習一",
        題目: 內建 {
            編碼: 並擊("yang'wang'xing'kong yan'jiu'jue'ding \
                        zi'ran'xuan'ze che'di'jue'lie \
                        bu'duan'fa'zhan ren'min'qun'zhong \
                        zhu'yi'an'quan huan'yuan'fan'ying \
                        sheng'ming'cai'chan shao'xiao'jun'xian \
                        yuan'lai'ru'ci hou'yan'wu'chi"),
            字幕: 詞句("仰望星空 研究決定 自然選擇 徹底決裂 不斷發展 人民羣衆 \
                        注意安全 還原反應 生命財產 少校軍銜 原來如此 厚顏無恥"),
        },
    },
    練習題 {
        標題: "綜合練習二",
        題目: 求取 { 網址: "/typewriter/static/drill/zonghe-lianxi-2.txt" },
    },
    練習題 {
        標題: "綜合練習三",
        題目: 求取 { 網址: "/typewriter/static/drill/zonghe-lianxi-3.txt" },
    },
    練習題 {
        標題: "縮略碼示例",
        題目: 內建 {
            編碼: 並擊("SHG=<shen me> SHGUA=<shu ru fa> ZUAO=<zui hao> \
                        FBUR=<wei shen me> ZFB=<mei you> BUA=<ban fa> SHGU=<shu ru> SGIAN=<shi jian> \
                        xian wen SHB=<shi bu shi> zai wen FBUR=<wei shen me>"),
            字幕: 詞句("[什麼] [輸入法] [最好] \
                        [爲什麼] [沒有] [辦法] [輸入] [時間] \
                        先問[是不是] 再問[爲什麼]"),
        },
    },
    練習題 {
        標題: "宮保拼音並擊術",
        題目: 內建 {
            編碼: 並擊("gong bao pin yin bing ji shu \
                        ZIE=<jie> zou ming kuai you zhi guan \
                        yi ji yi ge zhong wen zi \
                        neng fen ping qiao yu ZIAN=<jian> tuan \
                        biao zhun she bei guang jian rong \
                        liu jian wu chong CI=<qi> zhi chan \
                        su ji CIAN=<qian> zong gui jian yi \
                        yin yun wan bian lie qin pan"),
            字幕: 詞句("宮保拼音並擊術 節奏明快又直觀 \
                        一擊一個中文字 能分平翹與尖團 \
                        標準設備廣兼容 六鍵無衝七指禪 \
                        速記千宗歸簡易 音韻萬變列琴盤"),
        },
    },
    練習題 {
        標題: "倉頡轉世賦",
        題目: 求取 { 網址: "/typewriter/static/drill/cangjiezhuanshifu.txt" },
    },
    練習題 {
        標題: "中州韻",
        題目: 求取 { 網址: "/typewriter/static/drill/zhongzhouyun.txt" },
    },
    練習題 {
        標題: "鼠鬚管",
        題目: 求取 { 網址: "/typewriter/static/drill/shuxuguan.txt" },
    },
];
