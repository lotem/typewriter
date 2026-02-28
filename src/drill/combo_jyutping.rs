use super::{
    練習題,
    題目來源::{內建, 求取},
};
use crate::gear::{
    assignment::碼表定義::並擊,
    caption::字幕格式::{段落, 詞句},
    caption::字幕步進::逐詞,
};

pub const 宮保粵拼練習題: &[練習題] = &[
    練習題 {
        標題: "林峯－出鞘",
        題目: 內建 {
            編碼: 並擊(
                "cung jin sai gaan mou pat hou bing hei \
                 jat faat ziu \
                 fuk jyu ging tin dei \
                 cing wo ji \
                 bat jan se hei \
                 fau wan gung bou jyu hung jung \
                 bin waan hei \
                 \
                 zung si hot mong siu jiu zau ngo lou \
                 paau heoi hei sai dik gim pou \
                 jiu zuk ngo mung ging ling tin zan nou \
                 doi gaa taai gou \
                 fung \
                 teng ngo fu ziu \
                 wan jik tai ngo wu faat \
                 po hung zung ceot ciu \
                 jung seon sam gik teoi hoi siu \
                 wai daai hung tou maan sai jik ziu jiu \
                 \
                 cung jin sai gaan mou pat hou bing hei \
                 jat faat ziu \
                 fuk jyu ging tin dei \
                 cing wo ji \
                 bat jan se hei \
                 hung ngaan gau mung faa bat zeon gim zoi fei \
                 \
                 san sau dou ho faan wan fuk jyu hau \
                 jat daan so paan gaai dou sau \
                 wai geoi taa jat cing wong cing baa hau \
                 mut jau deoi sau \
                 sau hing on dou soeng \
                 mou noi gok bui fu liu naa bat bin laap coeng \
                 zeon liu cin bui liu jan ji \
                 gei si naam ji zyut deoi mei teoi joeng \
                 \
                 cung jin sai gaan mou pat hou bing hei \
                 jat faat ziu \
                 fuk jyu ging tin dei \
                 cing wo ji \
                 bat jan se hei \
                 sau jan wai sam bin zi gei \
                 \
                 cung jin sai gaan mou pat hou bing hei \
                 jat faat ziu \
                 fuk jyu ging tin dei \
                 cing wo ji \
                 bat jan se hei \
                 hung ngaan gau mung faa bat zeon gim zoi fei",
            ),
            字幕: 詞句(
                "重現世間無匹好兵器 一發招 覆雨驚天地 \
                 情和義 不忍捨棄 浮雲共暴雨洶湧 變幻起 \
                 縱是渴望逍遙走我路 拋去稀世的劍譜 \
                 要逐我夢竟令天震怒 代價太高 \
                 風 聽我呼召 雲亦替我護法 破空中出鞘 \
                 用信心擊退海嘯 偉大雄圖萬世亦照耀 \
                 重現世間無匹好兵器 一發招 覆雨驚天地 \
                 情和義 不忍捨棄 紅顏舊夢化不盡劍在飛 \
                 伸手都可翻雲覆雨後 一旦所盼皆到手 \
                 畏懼他日稱王稱霸後 沒有對手 \
                 手 輕按刀上 無奈各背負了那不變立場 \
                 盡了千杯了恩義 既是男兒絕對未退讓 \
                 重現世間無匹好兵器 一發招 覆雨驚天地 \
                 情和義 不忍捨棄 仇人爲甚變知己 \
                 重現世間無匹好兵器 一發招 覆雨驚天地 \
                 情和義 不忍捨棄 紅顏舊夢化不盡劍在飛",
            ),
        },
    },
    練習題 {
        標題: "聲母韻母練習",
        題目: 內建 {
            編碼: 並擊(
                "b p m f d t n l \
                 g k ng h gw kw w \
                 z c s j \
                 aa e [i]=<i> o [u]=<u> oe yu \
                 aai ai ei oi [u-i]=<ui> eoi \
                 aau au eu [i-u]=<iu> ou \
                 aam am em [i-m]=<im> \
                 aan an en [i-n]=<in> on [u-n]=<un> eon yun \
                 aang ang eng [i-ng]=<ing> ong [u-ng]=<ung> oeng \
                 aap ap ep [i-p]=<ip> \
                 aat at et [i-t]=<it> ot [u-t]=<ut> eot yut \
                 aak ak ek [i-k]=<ik> ok [u-k]=<uk> oek",
            ),
            字幕: 段落(
                逐詞,
                "b p m f d t n l
g k ng h gw kw w
z c s j
aa e i o u oe yu
aai ai ei oi ui eoi
aau au eu iu ou
aam am em im
aan an en in on un eon yun
aang ang eng ing ong ung oeng
aap ap ep ip
aat at et it ot ut eot yut
aak ak ek ik ok uk oek",
            ),
        },
    },
    練習題 {
        標題: "馬德鍾－江山",
        題目: 求取 {
            網址: "/typewriter/static/drill/gongsaan.txt",
        },
    },
    練習題 {
        標題: "陳慧嫻－千千闋歌",
        題目: 求取 {
            網址: "/typewriter/static/drill/cincinkyutgo.txt",
        },
    },
    練習題 {
        標題: "劉德華－一起走過的日子",
        題目: 求取 {
            網址: "/typewriter/static/drill/jatheizaugwodikjatzi.txt",
        },
    },
];
