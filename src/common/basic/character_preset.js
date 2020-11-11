import { supportedCharacters } from "genshin_panel";
import { chsCharacter } from "@/common/chs";

let _preset = {
    "preset": {
        "chs": "预设",
        "characters": {}
    }
};

const support = supportedCharacters();
for (let i = 0; i < support.length; i++) {
    let item = support[i];
    let temp = item.split("-");
    
    let chs = chsCharacter(temp[0]);
    let characters = _preset.preset.characters;

    let s1 = chs;
    let s2 = temp[1] + "级";
    let s3 = temp[2] === 0 ? "未突破" : "突破";

    if (chs === null) {
        continue;
    }

    if (characters[temp[0]]) {
        characters[temp[0]].presets.push({
            value: item,
            title: s1 + s2 + s3,
        })
    } else {
        characters[temp[0]] = {
            chs,
            presets: [
                {
                    value: item,
                    title: s1 + s2 + s3,
                }
            ]
        }
    }
}

Object.freeze(_preset);
export const preset = _preset;

// export const preset = {
//     "thunder": {
//         "chs": "雷",
//         "characters": {
//             "keqing": {
//                 "chs": "刻晴",
//                 "presets": [
//                     {
//                         value: "keqing-70-0",
//                         title: "刻晴70级未突破",
//                     },
//                 ]
//             },
//             "beidou": {
//                 "chs": "北斗",
//                 "presets": [
//                     {
//                         value: "beidou-70-0",
//                         title: "北斗70级未突破",
//                     }
//                 ]
//             },
//             "lisha": {
//                 "chs": "丽莎",
//                 "presets": [
//                     {
//                         value: "lisha-70-0",
//                         title: "丽莎70级未突破",
//                     }
//                 ]
//             },
//             "leize": {
//                 "chs": "雷泽",
//                 "presets": [
//                     {
//                         value: "leize-80-0",
//                         title: "雷泽80级未突破"
//                     }
//                 ]
//             },
//             "feixieer": {
//                 "chs": "菲谢尔",
//                 "presets": [
//                     {
//                         value: "feixieer-70-0",
//                         title: "菲谢尔70级未突破"
//                     }
//                 ]
//             },
//         }
//     },
//     "fire": {
//         "chs": "火",
//         "characters": {
//             "diluke": {
//                 "chs": "迪卢克",
//                 "presets": [
//                     {
//                         value: "diluke-70-0",
//                         title: "迪卢克70级未突破",
//                     }
//                 ]
//             },
//             "keli": {
//                 "chs": "可莉",
//                 "presets": [
//                     {
//                         value: "keli-70-0",
//                         title: "可莉70级未突破"
//                     }
//                 ]
//             },
//             "anbo": {
//                 "chs": "安柏",
//                 "presets": [
//                     {
//                         value: "anbo-70-0",
//                         title: "安柏70级未突破"
//                     }
//                 ]
//             },
//             "bannite": {
//                 "chs": "班尼特",
//                 "presets": [
//                     {
//                         value: "bannite-70-0",
//                         title: "班尼特70级未突破"
//                     }
//                 ]
//             },
//             "xiangling": {
//                 "chs": "香菱",
//                 "presets": [
//                     {
//                         value: "xiangling-70-0",
//                         title: "香菱70级未突破"
//                     }
//                 ]
//             },
//         }
//     },
//     "ice": {
//         "chs": "冰",
//         "characters": {
//             "chongyun": {
//                 "chs": "重云",
//                 "presets": [
//                     {
//                         value: "chongyun-70-0",
//                         title: "重云70级未突破"
//                     }
//                 ]
//             },
//             "kaiya": {
//                 "chs": "凯亚",
//                 "presets": [
//                     {
//                         value: "kaiya-70-0",
//                         title: "凯亚70级未突破"
//                     }
//                 ]
//             },
//             "qiqi": {
//                 "chs": "七七",
//                 "presets": [
//                     {
//                         value: "qiqi-70-0",
//                         title: "七七70级未突破"
//                     }
//                 ]
//             },
//         }
//     },
//     "rock": {
//         "chs": "岩",
//         "characters": {
//             "ningguang": {
//                 "chs": "凝光",
//                 "presets": [
//                     {
//                         value: "ningguang-70-0",
//                         title: "凝光70级未突破"
//                     }
//                 ]
//             },
//             "nuoaier": {
//                 "chs": "诺艾尔",
//                 "presets": [
//                     {
//                         value: "nuoaier-70-0",
//                         title: "诺艾尔70级未突破"
//                     }
//                 ]
//             },
//         },
//     },
//     "water": {
//         "chs": "水",
//         "characters": {
//             "babala": {
//                 "chs": "芭芭拉",
//                 "presets": [
//                     {
//                         value: "babala-70-0",
//                         title: "芭芭拉70级未突破"
//                     }
//                 ]
//             },
//             "mona": {
//                 "chs": "莫娜",
//                 "presets": [
//                     {
//                         value: "mona-70-0",
//                         title: "莫娜70级未突破"
//                     }
//                 ]
//             },
//             "xingqiu": {
//                 "chs": "行秋",
//                 "presets": [
//                     {
//                         value: "xingqiu-70-0",
//                         title: "行秋70级未突破"
//                     }
//                 ]
//             },
//         }
//     },
//     "wind": {
//         "chs": "风",
//         "characters": {
//             "qin": {
//                 "chs": "琴",
//                 "presets": [
//                     {
//                         value: "qin-70-0",
//                         title: "琴70级未突破"
//                     }
//                 ]
//             },
//             "shatang": {
//                 "chs": "砂糖",
//                 "presets": [
//                     {
//                         value: "shatang-70-0",
//                         title: "砂糖70级未突破"
//                     }
//                 ]
//             },
//             "wendi": {
//                 "chs": "温迪",
//                 "presets": [
//                     {
//                         value: "wendi-70-0",
//                         title: "温迪70级未突破"
//                     }
//                 ]
//             },
//         }
//     }
// }

// let _presetMap = {};
// for (let ele in preset) {
//     for (let ch in preset[ele].characters) {
//         for (p in preset[ele].characters[ch].presets) {
//             let name = p.value;

//         }
//     }
// }