use super::练习题;
use crate::gear::caption::{
    字幕格式::{段落, 词句},
    字幕步进::逐字,
};

pub const 小鹤双拼练习题: &[练习题] = &[
    练习题 {
        标题: "聪明的输入法懂我心意",
        编码: "cong ming de shu ru fa dong wo xin yi",
        字幕: 词句("聪明的输入法懂我心意"),
    },
    练习题 {
        标题: "出师表",
        编码: "xian di chuang ye wei ban er zhong dao beng cu \
                jin tian xia san fen yi zhou pi bi ci cheng wei ji cun wang zhi qiu ye \
                ran shi wei zhi chen bu xie yu nei zhong zhi zhi shi wang shen yu wai zhe \
                gai zhui xian di zhi yu yu bao zhi yu xia ye \
                cheng yi kai sheng ting yi guang xian di yi de hui hong zhi shi chen \
                bu yi wang zi fei bo yin yu shi yi yi sai zhong jian zhi lu ye \
                gong zhong fu zhong ju wei yi ti zang fa zang fou bu yi yi tong \
                shi you zuo jian ji ling zhong zhe xing zhong yan you si ji qi xing lun qi xing shang \
                zuo zhong bu xing bu zhi yi zhi zhong jian zhi que lou you suo guang yi \
                shi shi shi chu zhi chen xing xing su su bu chi yu dang shi zhe \
                jie liang zhong zhi chen ye yuan bi xia qin zhi xin zhi ze han shi zhi long ke ji ri er dai ye \
                chen ben bu yi gong geng yu nan yang \
                gou quan xing ming yu luan shi bu qiu wen da zhu hou \
                xian di bu yi chen bei biie wang zi qu qu zhi \
                san gu chen yu cao lu zhi zhong zi chen yi dang shi zhi shi \
                you shi gan ji sui xu xian di qu chi hou jin zhen fu dao yuan \
                hou zhi jun ju er dao ling bing zhan wei jue xian ming bu xiao \
                jin tian xia ding yi yi zhou pi bi ci cheng wei ji cun wang zhi qiu ye \
                ran shi wei zhi chen bu xie yu nei zhong zhi zhi shi wang shen yu wai zhe \
                gai zhui xian di zhi yu yu bao zhi yu xia ye \
                cheng yi kai sheng ting yi guang xian di yi de hui hong zhi shi chen \
                bu yi wang zi fei bo yin yu shi yi yi sai zhong jian zhi lu ye \
                gong zhong fu zhong ju wei yi ti zang fa zang fou bu yi yi tong \
                shi you zuo jian ji ling zhong zhe xing zhong yan you si ji qi xing lun qi xing shang \
                zuo zhong bu xing bu zhi yi zhi zhong jian zhi que lou you suo guang yi \
                shi shi shi chu zhi chen xing xing su su bu chi yu dang shi zhe \
                jie liang zhong zhi chen ye yuan bi xia qin zhi xin zhi ze han shi zhi long ke ji ri er dai ye \
                chen ben bu yi gong geng yu nan yang \
                gou quan xing ming yu luan shi bu qiu wen da zhu hou \
                xian di bu yi chen bei biie wang zi qu qu zhi \
                san gu chen yu cao lu zhi zhong zi chen yi dang shi zhi shi \
                you shi gan ji sui xu xian di qu chi hou jin zhen fu dao yuan \
                hou zhi jun ju er dao ling bing zhan wei jue xian ming bu xiao \
                jin dang yuan li zhi bing fa zui qiang chu ming bu xiao er ren yong zhi \
                shi wei zhi chen qu ci zhi chen ye zhi bi xia shen zhi yi \
                yi zhi ming chui zhuo que shi bu xiao ze ming zhi chen zhi zui ye \
                qin bi xia tuo chen yi tao zei xing fu xiao xiao zhi guo yi zhao ping ming \
                chen bu sheng shou en gan ji jin dang yuan li lin biao ti ling bu zhi suo yan",
        字幕: 段落(逐字, "先帝创业未半而中道崩殂 今天下三分 益州疲弊 此诚危急存亡之秋也 
                        然侍卫之臣不懈于内 忠志之士忘身于外者 盖追先帝之遇欲报之于陛下也
                        诚宜开张圣听 以光先帝遗德 恢弘志士之气 不宜妄自菲薄 引喻失义 以塞忠谏之路也
                        宫中府中俱为一体 陟罚臧否不宜异同
                        若有作奸犯科及为忠善者 宜付有司论其刑赏 以昭陛下平明之治
                        不宜偏私 使内外异法也
                        侍中侍郎郭攸之费祎董允等 此皆良实 志虑忠纯 是以先帝简拔以遗陛下
                        愚以为宫中之事 事无大小 悉以咨之 然后施行 必能裨补阙漏 有所广益也
                        将军向宠 性行淑均 晓畅军事 试用之于昔日 先帝称之曰能 是以众议举宠为督
                        愚以为营中之事 悉以咨之 必能使行阵和穆 优劣得所也
                        亲贤臣 远小人 此先汉所以兴隆也 亲小人 远贤臣 此后汉所以倾颓也
                        先帝在时 每与臣论此事 未尝不叹息痛恨于桓灵也
                        侍中尚书长史参军 此悉贞亮死节之臣也 愿陛下亲之信之 则汉室之隆可计日而待也
                        臣本布衣 躬耕于南阳 苟全性命于乱世 不求闻达于诸侯
                        先帝不以臣卑鄙 猥自枉屈 三顾臣于草庐之中 咨臣以当世之事
                        由是感激 遂许先帝以驱驰 后值倾覆 受任于败军之际 奉命于危难之间
                        尔来二十有一年矣
                        先帝知臣谨慎 故临崩寄臣以大事也
                        受命以来 夙夜忧叹 恐付托不效 以伤先帝之明 故五月渡泸 深入不毛
                        今南方已定 兵甲已足 当奖率三军 北定中原 庶竭驽钝 攘除奸凶 兴复汉室 还于旧都
                        此臣所以报先帝而忠陛下之职分也 至于斟酌损益 进尽忠言 则攸之祎允之任也
                        愿陛下托臣以讨贼兴复之效 不效则治臣之罪 以告先帝之灵
                        若无兴德之言 则责攸之祎允等之慢 以彰其咎
                        陛下亦宜自谋 以咨诹善道 察纳雅言 深追先帝遗诏 
                        臣不胜受恩感激 今当远离 临表涕零 不知所言")
    }
];
