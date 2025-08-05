use super::練習題;
use crate::gear::caption::{
    字幕格式::{段落, 詞句},
    字幕步進::逐詞,
};

pub const 宮保拼音練習題: &[練習題] = &[
    練習題 {
        標題: "中州韻輸入法引擎",
        編碼: "zhong zhou yun shu ru fa yin qing \
               si xiang yong jian pan biao da ye xing",
        字幕: 詞句("中州韻輸入法引擎 思想用鍵盤表達也行"),
    },
    練習題 {
        標題: "東風破早梅",
        編碼: "dong feng po zao mei \
               xiang nuan yi zhi kai \
               bing xue wu ren jian \
               chun cong tian shang lai",
        字幕: 詞句("東風破早梅 向暖一枝開 冰雪無人見 春從天上來"),
    },
    練習題 {
        標題: "聲母韻母練習",
        編碼: "b p m f d t n l \
               g k h j q x \
               zh ch sh r z c s \
               i u ü \
               A=a ia ua o uo e ie üe er \
               ai uai ei uei ao iao ou iou \
               an ian uan üan en in uen ün \
               ang iang uang eng ing ueng ong iong",
        字幕: 段落(逐詞,
                   "b p m f d t n l
                    g k h j q x
                    zh ch sh r z c s
                    i u ü
                    a ia ua o uo e ie üe er
                    ai uai ei uei ao iao ou iou
                    an ian uan üan en in uen ün
                    ang iang uang eng ing ueng ong iong"),
    },
    練習題 {
        標題: "音節練習",
        編碼: "bu pu fu me \
               de te le ne \
               ge ke he \
               zhi chi shi ri zi ci si \
               er AE=<'a> A=<␣>",
        字幕: 段落(逐詞,
                   "bu pu fu me
                    de te le ne
                    ge ke he
                    zhi chi shi ri zi ci si
                    er 'a ␣"),
    },
    練習題 {
        標題: "單字練習",
        編碼: "yi er san si wu liu qi ba jiu shi \
               ren kou shou shang zhong xia ri yue shui huo shan shi tian tu \
               mu he mi zhu dao gong che zhou qian hou zuo you er mu she ya \
               tou zu zuo li zou chi cun yuan jiao fen \
               da xiao duo shao jin liang ma niu yang mao pi \
               niao chong yu zhua zi wei ba kai men guan chu ru lai qu \
               fang xiang dong nan xi bei bai tian hei ye yun dian feng yu \
               fu qin mu er nü ba ma",
        字幕: 詞句("一二三四五 六七八九十 \
                    人口手 上中下 日月水火 山石田土 \
                    木禾米竹 刀弓車舟 前後左右 耳目舌牙 \
                    頭足坐立走 尺寸元角分 \
                    大小多少斤兩 馬牛羊毛皮 \
                    鳥蟲魚爪子尾巴 開門關出入來去 \
                    方向東南西北 白天黑夜雲電風雨 \
                    父親母兒女爸媽"),
    },
    練習題 {
        標題: "綜合練習一",
        編碼: "yang'wang'xing'kong yan'jiu'jue'ding \
               zi'ran'xuan'ze che'di'jue'lie \
               bu'duan'fa'zhan ren'min'qun'zhong \
               zhu'yi'an'quan huan'yuan'fan'ying \
               sheng'ming'cai'chan shao'xiao'jun'xian \
               yuan'lai'ru'ci hou'yan'wu'chi",
        字幕: 詞句("仰望星空 研究決定 自然選擇 徹底決裂 不斷發展 人民羣衆 \
                    注意安全 還原反應 生命財產 少校軍銜 原來如此 厚顏無恥"),
    },
    練習題 {
        標題: "綜合練習二",
        編碼: "wan'sui jie'jue guo'cheng quan'xuan tiao'jian yuan'yin sheng'chan'xian \
               tu'di nu'li he'ge qi'ji tu'chu tu'shu wan'quan yuan'yan wan'yan yan'yuan \
               zhi'chi fu'chi fu'zhi fu'shi si'shi shi'si si'shi'si \
               wu'yi yi'wu yu'yi yi'yu wu'yu yu'wu wu'wu yi'yi yu'yu \
               chang'shang shang'chang shi'chang shi'shang zhi'zhang zhi'chang shi'zhang chang'zhang \
               fang'zhang chang'lang shang'hang cang'sang sang'zang bang'mang lang'dang rang'rang \
               tang'lang yuan'yang yong'yuan yong'you yuan'you yuan'wen wei'wen wen'zhang wen'dang \
               lun'wen nian'lun yan'lun duan'dian duan'lian zhuan'huan chuan'huan lun'huan \
               guan'jun guan'li'yuan chuan'yuan zhan'zhuan shan'chuan zhan'chuan",
        字幕: 詞句("萬歲 解決 過程 全選 條件 原因 生產綫 \
                    土地 努力 合格 奇跡 突出 圖書 完全 怨言 晚宴 演員 \
                    支持 扶持 複製 服飾 四十 十四 四十四 \
                    無疑 義務 雨衣 易於 無語 雨霧 無誤 意義 遇雨 \
                    廠商 商場 市場 時尚 紙張 職場 市長 廠長 \
                    方丈 長廊 商行 滄桑 喪葬 幫忙 浪蕩 嚷嚷 \
                    螳螂 鴛鴦 永遠 擁有 原有 原文 慰問 文章 文檔 \
                    論文 年輪 言論 斷電 鍛鍊 轉換 傳喚 輪換 \
                    冠軍 管理員 船員 輾轉 山川 戰船"),
    },
    練習題 {
        標題: "綜合練習三",
        編碼: "jing'xing gong'jing jiong'jing hong'xing jing'kong kong'jing qiong'kong qiong'kun \
               wang'jing guang'jing kuang'jing kuang'wang qiang'guang qiang'jiang qiang'xiang \
               yang'xiang yang'qiang xiang'xiang bei'jing bei'ying bei'qing fei'xing \
               bei'fang bei'yang bei'fei fei'fan fei'chang fei'fu hui'fei hui'yi xue'hui \
               yi'wei you'wei you'you you'yao yao'you yao'wu yao'yi wo'ye'you ye'yao'you \
               e'yi e'wu e'yu ji'e qi'e xi'ji xi'qi qi'xi xi'wu si'xu ci'xu zi'xu \
               qu'ju qu'qu qu'yu qi'yi gu'wu gu'yi ji'yi ju'yu yu'ju yu'ji \
               shuo'guo zhong'guo zui'duo huo'zhe ge'jiu'ge'wei yue'lai'yue \
               wei'shen'me zen'me'yang ke'bu'ke'yi you'mei'you",
        字幕: 詞句("驚醒 恭敬 窘境 紅杏 驚恐 孔徑 穹空 窮困 \
                    網警 光景 礦井 狂妄 強光 強將 強項 \
                    洋相 洋槍 想象 北京 背影 悲情 飛行 \
                    北方 北洋 北非 非凡 非常 肺腑 會飛 會議 學會 \
                    以爲 尤爲 又有 又要 要有 藥物 要義 我也有 也要有 \
                    惡意 訛誤 俄語 飢餓 企鵝 襲擊 稀奇 七夕 習武 思緒 次序 自詡 \
                    屈居 區區 區域 起義 鼓舞 故意 記憶 局域 語句 預計 \
                    說過 中國 最多 或者 各就各位 越來越 \
                    爲什麼 怎麼樣 可不可以 有沒有"),
    },
    練習題 {
        標題: "縮略碼示例",
        編碼: "SHG=<shen me> SHGUA=<shu ru fa> ZUAO=<zui hao> \
               FBUR=<wei shen me> ZFB=<mei you> BUA=<ban fa> SHGU=<shu ru> SGIAN=<shi jian> \
               xian wen SHB=<shi bu shi> zai wen FBUR=<wei shen me>",
        字幕: 詞句("[什麼] [輸入法] [最好] \
                    [爲什麼] [沒有] [辦法] [輸入] [時間] \
                    先問[是不是] 再問[爲什麼]"),
    },
    練習題 {
        標題: "宮保拼音並擊術",
        編碼: "gong bao pin yin bing ji shu \
               zie zou ming kuai you zhi guan \
               yi ji yi ge zhong wen zi \
               neng fen ping qiao yu zian tuan \
               biao zhun she bei guang jian rong \
               liu jian wu chong cii zhi chan \
               su ji cian zong gui jian yi \
               yin yun wan bian lie qin pan",
        字幕: 詞句("宮保拼音並擊術 節奏明快又直觀 \
                    一擊一個中文字 能分平翹與尖團 \
                    標準設備廣兼容 六鍵無衝七指禪 \
                    速記千宗歸簡易 音韻萬變列琴盤"),
    },
    練習題 {
        標題: "倉頡轉世賦",
        編碼: "cang jie gong zhuan shi \
               qiao ziang chu he yin \
               zhong zhou yun qi yan shi fan pin \
               da dao zhi jian yi xuan ao gao shen \
               yun yong miao fa cun hu yi sin \
               zong cing yu lie zi hai ci lin \
               hui bu hui mou tian chuan yue gu jin \
               kou zhong yan yu shi hua xia zheng yin \
               wo zen ken ziang zhu liu fang ren \
               ku zhi yi nian zhong bu de siao sa \
               nan yu jiang ming liao sin li zhen hua \
               ji cing bian ma yong cian wan ci qiao da \
               xue hui si siang zai jian pan shang biao da",
        字幕: 詞句("蒼頡公轉世 巧匠出河陰 中州韻豈言是凡品 \
                    大道至簡亦玄奧高深 運用妙法存乎一心 \
                    縱情漁獵字海詞林 會不會某天穿越古今 \
                    口中言語是華夏正音 我怎肯將主流放任 \
                    苦執一念終不得瀟灑 難於講明了心裏真話 \
                    寄情編碼用千萬次敲打 學會思想在鍵盤上表達"),
    },
    練習題 {
        標題: "中州韻",
        編碼: "hui hao ji jian shu tu tong gui \
               wen zi zing miao ji qiao sheng hui \
               shu fa cheng shi bu dao chen gui \
               jin yin gu yun yan yi lui tui \
               ji si zhong tu sin huai zhu yi \
               die dai han shu da zao shen qi \
               dai ma bu ji zhi ling sii li \
               kai bi yuan xi si liu yong ji",
        字幕: 詞句("揮毫擊鍵 殊途同歸 文字精妙 機巧生輝 \
                    書法程式 不蹈陳規 今音古韻 演繹類推 \
                    寄思中土 心懷主義 迭代寒暑 打造神器 \
                    代碼不羈 指令犀利 開彼源兮 斯流永繼"),
    },
    練習題 {
        標題: "鼠鬚管",
        編碼: "zhuo jun yi jing zhou yu zhen zhi ziao \
               zeng jun yi süan cheng shu sü zhi guan \
               ziu ru chang hong yin cang hai \
               bi ruo zün ma chi ping ban \
               ai jun shang shao li fang hao \
               zie wo jiu shuai huan zian sian \
               wen zhang jing shi zhi ming zao \
               yi qi lun jiao siang de wan \
               yu zhen ziao \
               yi ju shi fen dang fu zhan \
               shu sü guan \
               wei wu sui wei cing bu cian \
               sin shi zui mo shi yi hui \
               bie hou ji wo wu ci yuan",
        字幕: 詞句("酌君以荊州魚枕之蕉 贈君以宣城䑕須之管 \
                    酒如長虹飲滄海 筆若駿馬馳平坂 \
                    愛君尚少力方豪 嗟我久衰歡漸鮮 \
                    文章驚世知名早 意氣論交相得晚 \
                    魚枕蕉 一舉十分當覆盞 \
                    䑕須管 為物雖㣲情不淺 \
                    新詩醉墨時一揮 別後寄我無辭逺"),
    },
];
