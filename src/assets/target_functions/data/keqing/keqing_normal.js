import badge from "./badge.png";
import createFreqBasedDamageFunction from "../factory/freq_based_func";

let args = {
    element: "thunder",

    aFreq: 60,
    aRatio: 0.7,
    aTimes: 0.8522,

    bFreq: 20,
    bRatio: 0.9,
    bTimes: 2.57,

    eFreq: 15,
    eTimes: 3.276,

    qFreq: 5,
    qTimes: 6.558
}

// let args = {
//     element: "thunder",

//     aFreq: 1,
//     aRatio: 0,
//     aTimes: 0.8522,

//     bFreq: 0,
//     bRatio: 0.9,
//     bTimes: 2.57,

//     eFreq: 0,
//     eTimes: 3.276,

//     qFreq: 0,
//     qTimes: 6.558
// }

export default {
    name: "keqingNormal",
    chs: "雷伤刻晴",
    description: [
        "非常普通的输出手法",
        "2段E",
        "平均3A1重击",
    ],
    tags: [
        "输出",
        "刻晴",
        "元素伤害",
    ],
    func: createFreqBasedDamageFunction(args),
    "for": "keqing",
    badge,
}