import badge from "./badge.png";
import createFreqBasedDamageFunction from "../factory/freq_based_func";

let args = {
    element: "thunder",

    aFreq: 25,
    aRatio: 0,
    aTimes: 0.8522,

    bFreq: 55,
    bRatio: 0,
    bTimes: 2.57,

    eFreq: 10,
    eTimes: 3.276,

    qFreq: 5,
    qTimes: 6.558
}

export default {
    name: "keqingPhysical",
    chs: "刻晴-物晴",
    description: [
        "1段E+重击",
        "平均1A3重击",
    ],
    tags: [
        "输出",
        "刻晴",
        "物理",
    ],
    func: createFreqBasedDamageFunction(args),
    "for": "keqing",
    badge,
}