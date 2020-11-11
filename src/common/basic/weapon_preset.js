import { supportedWeapons } from "genshin_panel";
import { chsWeapon } from "@/common/chs";

// export const preset = {
//     "sword1": {
//         "chs": "单手剑",
//         "weapons": {
//             "antiejian": {
//                 "chs": "暗铁剑",
//                 "presets": [
//                     {
//                         value: "antiejian-20-0",
//                         title: "暗铁剑20级未突破",
//                     },
//                 ],
//             },
//             "chihuyudao": {
//                 "chs": "吃虎鱼刀",
//                 "presets": [
//                     {
//                         value: "chihuyudao-70-0",
//                         title: "吃虎鱼刀70级未突破",
//                     },
//                 ],
//             },
//             "dijian": {
//                 "chs": "笛剑",
//                 "presets": [
//                     {
//                         value: "dijian-70-0",
//                         title: "笛剑70级未突破",
//                     },
//                 ],
//             },
//             "feitianyujian": {
//                 "chs": "飞天御剑",
//                 "presets": [
//                     {
//                         value: "feitianyujian-40-0",
//                         title: "飞天御剑40级未突破",
//                     },
//                 ],
//             },
//             "fengyingjian": {
//                 "chs": "飞天御剑",
//                 "presets": [
//                     {
//                         value: "feitianyujian-40-0",
//                         title: "飞天御剑40级未突破",
//                     },
//                 ],
//             },
//             "heijian": {
//                 "chs": "黑剑",
//                 "presets": [
//                     {
//                         value: "heijian-70-0",
//                         title: "黑剑70级未突破",
//                     },
//                 ],
//             },
//             "limingshenjian": {
//                 "chs": "黎明神剑",
//                 "presets": [
//                     {
//                         value: "limingshenjian-70-0",
//                         title: "黎明神剑70级未突破"
//                     },
//                 ]
//             },
//             "xialilongyin": {
//                 "chs": "匣里龙吟",
//                 "presets": [
//                     {
//                         value: "xialilongyin-70-0",
//                         title: "匣里龙吟70级未突破"
//                     },
//                     // {
//                     //     value: "xialilongyin-70-0-b",
//                     //     title: "匣里龙吟70级未突破，计入效果"
//                     // }
//                 ]
//             },
//             "xifengjian": {
//                 "chs": "西风剑",
//                 "presets": [
//                     {
//                         value: "xifengjian-70-0",
//                         title: "西风剑70级未突破"
//                     },
//                 ]
//             }
//         }
//     }
// };

let _preset = {
    "preset": {
        "chs": "预设",
        "weapons": {

        }
    }
};

const support = supportedWeapons();
for (let i = 0; i < support.length; i++) {
    let item = support[i];
    let temp = item.split("-");
    // window.console.log(temp);
    let chs = chsWeapon(temp[0]);
    let weapons = _preset.preset.weapons;

    let s1 = chs;
    let s2 = temp[1] + "级";
    let s3 = temp[2] === 0 ? "未突破" : "突破";

    if (chs === null) {
        continue;
    }

    if (weapons[temp[0]]) {
        weapons[temp[0]].presets.push({
            value: item,
            title: s1 + s2 + s3,
        })
    } else {
        weapons[temp[0]] = {
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

// window.console.log(_preset);
Object.freeze(_preset);
export const preset = _preset;