use leptos::*;

use crate::assignment::作業;

struct 字幕指標<'a> {
    字幕: &'a str,
    指標: usize,
}

impl<'a> From<&'a str> for 字幕指標<'a> {
    fn from(字幕: &'a str) -> Self {
        Self { 字幕, 指標: 0 }
    }
}

/// 迭代字幕中的文字.
/// 傳入的字幕應當是從空白處切分出的一段.
/// 通常一音對一字. 例外情況用文字組標記 `[]` 括住與一個音節對應的一組文字.
/// 文字組不能包含空白字符及左右方括號.
impl Iterator for 字幕指標<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut 剩餘文字 = self.字幕.chars().skip(self.指標);
        match 剩餘文字.next() {
            Some('[') => {
                // 將文字組標記 [] 中的文字串視作一個文字
                let 文字組 = 剩餘文字.take_while(|字| *字 != ']');
                self.指標 += 文字組.clone().count() + 2;
                Some(文字組.collect())
            }
            Some(單字) => {
                self.指標 += 1;
                Some(單字.to_string())
            }
            None => None,
        }
    }
}

#[component]
pub fn Rime字幕屏(當前作業: Signal<作業>, 作業進度: Signal<usize>) -> impl IntoView {
    let 分段字幕 = create_memo(move |_| {
        當前作業.with(|作業| {
            作業.字幕().map(move |有字幕| {
                有字幕
                    .split_whitespace()
                    .fold(
                        (0, Box::new(vec![])),
                        |(起始字序, mut 已標註字序的段落), 又一段| {
                            let 結束字序 = 起始字序 + 字幕指標::from(又一段).count();
                            (*已標註字序的段落).push((起始字序, 結束字序, 又一段));
                            (結束字序, 已標註字序的段落)
                        },
                    )
                    .1
            })
        })
    });

    let 該段字幕按進度顯示 = move || {
        分段字幕.with(|有冇分段字幕| {
            有冇分段字幕.as_ref().and_then(|衆段落| {
                let 全文進度 = 作業進度();
                let 當前段落號 =
                    衆段落.partition_point(|(_, 段落結束, _)| *段落結束 <= 全文進度);
                衆段落.get(當前段落號).map(|當前段落| {
                    let (段落起始, _, 段落文字) = 當前段落;
                    let 段落進度 = 全文進度 - 段落起始;
                    let 完成的字 = 字幕指標::from(*段落文字).take(段落進度).collect::<String>();
                    let 當下的字 = 字幕指標::from(*段落文字)
                        .skip(段落進度)
                        .take(1)
                        .collect::<String>();
                    let 剩餘的字 = 字幕指標::from(*段落文字)
                        .skip(段落進度 + 1)
                        .collect::<String>();
                    (完成的字, 當下的字, 剩餘的字)
                })
            })
        })
    };

    view! {
        <div class="text-box">
            <div class="caption">
            {
                move || 該段字幕按進度顯示().map(|(完成的字, 當下的字, 剩餘的字)| view! {
                    <span class="accepted">{完成的字}</span>
                    <span class="highlight">{當下的字}</span>
                    <span>{剩餘的字}</span>
                })
            }
            </div>
        </div>
    }
}
