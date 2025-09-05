use super::練習題;
use crate::gear::caption::字幕格式::詞句;

pub const 粵語練習題: &[練習題] = &[練習題 {
    標題: "林峯－出鞘",
    編碼: "[清東]=<cung> [以先]=<jin> [心威]=<sai> [見翻]=<gaan> \
           [明高]=<mou> [滂畢]=<pat> [曉高]=<hou> [幫英]=<bing> [曉幾]=<hei> \
           [以畢]=<jat> [非發]=<faat> [精朝]=<ziu> \
           [非篤]=<fuk> [以諸]=<jyu> [見英]=<ging> [透先]=<tin> [端幾]=<dei> \
           [清英]=<cing> [云科]=<wo> [以師]=<ji> \
           [幫畢]=<bat> [以賓]=<jan> [心遮]=<se> [曉幾]=<hei> \
           [非修]=<fau> [云賓]=<wan> [見東]=<gung> [幫高]=<bou> [以諸]=<jyu> [曉東]=<hung> [以東]=<jung> \
           [幫先]=<bin> [云翻]=<waan> [曉幾]=<hei> \
           \
           [精東]=<zung> [心師]=<si> [曉割]=<hot> [明剛]=<mong> \
           [心朝]=<siu> [以朝]=<jiu> [精修]=<zau> [我科]=<ngo> [來高]=<lou> \
           [滂交]=<paau> [曉雖]=<heoi> [曉幾]=<hei> [心威]=<sai> [端益]=<dik> [見兼]=<gim> [滂高]=<pou> \
           [以朝]=<jiu> [精篤]=<zuk> [我科]=<ngo> [明東]=<mung> \
           [見英]=<ging> [來英]=<ling> [透先]=<tin> [精賓]=<zan> [泥高]=<nou> \
           [端栽]=<doi> [見家]=<gaa> [透皆]=<taai> [見高]=<gou> \
           [非東]=<fung> [透鄭]=<teng> [我科]=<ngo> [非孤]=<fu> [精朝]=<ziu> \
           [云賓]=<wan> [以益]=<jik> [透威]=<tai> [我科]=<ngo> [云孤]=<wu> [非發]=<faat> \
           [滂科]=<po> [曉東]=<hung> [精東]=<zung> [清卒]=<ceot> [清朝]=<ciu> \
           [以東]=<jung> [心津]=<seon> [心金]=<sam> \
           [見益]=<gik> [透雖]=<teoi> [曉栽]=<hoi> [心朝]=<siu> \
           [云威]=<wai> [端皆]=<daai> [曉東]=<hung> [透高]=<tou> \
           [明翻]=<maan> [心威]=<sai> [以益]=<jik> [精朝]=<ziu> [以朝]=<jiu> \
           \
           [清東]=<cung> [以先]=<jin> [心威]=<sai> [見翻]=<gaan> \
           [明高]=<mou> [滂畢]=<pat> [曉高]=<hou> [幫英]=<bing> [曉幾]=<hei> \
           [以畢]=<jat> [非發]=<faat> [精朝]=<ziu> \
           [非篤]=<fuk> [以諸]=<jyu> [見英]=<ging> [透先]=<tin> [端幾]=<dei> \
           [清英]=<cing> [云科]=<wo> [以師]=<ji> \
           [幫畢]=<bat> [以賓]=<jan> [心遮]=<se> [曉幾]=<hei> \
           [曉東]=<hung> [我翻]=<ngaan> [見修]=<gau> [明東]=<mung> [非家]=<faa> [幫畢]=<bat> [精津]=<zeon> \
           [見兼]=<gim> [精栽]=<zoi> [非幾]=<fei> \
           \
           [心賓]=<san> [心修]=<sau> [端高]=<dou> [曉科]=<ho> \
           [非翻]=<faan> [云賓]=<wan> [非篤]=<fuk> [以諸]=<jyu> [曉修]=<hau> \
           [以畢]=<jat> [端翻]=<daan> [心科]=<so> [滂翻]=<paan> \
           [見皆]=<gaai> [端高]=<dou> [心修]=<sau> \
           [云威]=<wai> [見雖]=<geoi> [透家]=<taa> [以畢]=<jat> \
           [清英]=<cing> [云剛]=<wong> [清英]=<cing> [幫家]=<baa> [曉修]=<hau> \
           [明括]=<mut> [以修]=<jau> [端雖]=<deoi> [心修]=<sau> \
           [心修]=<sau> [曉英]=<hing> [亞干]=<on> [端高]=<dou> [心張]=<soeng> \
           [明高]=<mou> [泥栽]=<noi> [見角]=<gok> [幫魁]=<bui> [非孤]=<fu> [來朝]=<liu> \
           [泥家]=<naa> [幫畢]=<bat> [幫先]=<bin> [來甲]=<laap> [清張]=<coeng> \
           [精津]=<zeon> [來朝]=<liu> [清先]=<cin> [幫魁]=<bui> \
           [來朝]=<liu> [以賓]=<jan> [以師]=<ji> \
           [見幾]=<gei> [心師]=<si> [泥緘]=<naam> [以師]=<ji> \
           [精乙]=<zyut> [端雖]=<deoi> [明幾]=<mei> [透雖]=<teoi> [以張]=<joeng> \
           \
           [清東]=<cung> [以先]=<jin> [心威]=<sai> [見翻]=<gaan> \
           [明高]=<mou> [滂畢]=<pat> [曉高]=<hou> [幫英]=<bing> [曉幾]=<hei> \
           [以畢]=<jat> [非發]=<faat> [精朝]=<ziu> \
           [非篤]=<fuk> [以諸]=<jyu> [見英]=<ging> [透先]=<tin> [端幾]=<dei> \
           [清英]=<cing> [云科]=<wo> [以師]=<ji> \
           [幫畢]=<bat> [以賓]=<jan> [心遮]=<se> [曉幾]=<hei> \
           [心修]=<sau> [以賓]=<jan> [云威]=<wai> [心金]=<sam> [幫先]=<bin> [精師]=<zi> [見幾]=<gei> \
           \
           [清東]=<cung> [以先]=<jin> [心威]=<sai> [見翻]=<gaan> \
           [明高]=<mou> [滂畢]=<pat> [曉高]=<hou> [幫英]=<bing> [曉幾]=<hei> \
           [以畢]=<jat> [非發]=<faat> [精朝]=<ziu> \
           [非篤]=<fuk> [以諸]=<jyu> [見英]=<ging> [透先]=<tin> [端幾]=<dei> \
           [清英]=<cing> [云科]=<wo> [以師]=<ji> \
           [幫畢]=<bat> [以賓]=<jan> [心遮]=<se> [曉幾]=<hei> \
           [曉東]=<hung> [我翻]=<ngaan> [見修]=<gau> [明東]=<mung> [非家]=<faa> [幫畢]=<bat> [精津]=<zeon> \
           [見兼]=<gim> [精栽]=<zoi> [非幾]=<fei>",
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
}];

pub const 宮保粵拼練習題: &[練習題] = &[練習題 {
    標題: "林峯－出鞘",
    編碼: "cung jin sai gaan mou pat hou bing hei \
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
}];
