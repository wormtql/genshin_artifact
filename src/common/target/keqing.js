import { all } from "./base";

// const calc = elementalBasic("thunder", 0.7);

const calc = all({
    element: "thunder",
    aFreq: 60,
    aRatio: 0.7,
    aTimes: 0.8522,
    bFreq: 10,
    bRatio: 0.9,
    bTimes: 2.57,
    eFreq: 20,
    eTimes: 3.276,
    qFreq: 10,
    qTimes: 6.558
});

export const plans = [
    {
        value: "keqing-normal",
        name: "常规刻晴",
        calc: calc,
        description: "普通的输出型刻晴",
        tags: [
            "普攻频率0.6，元素比例0.7，倍率0.8522",
            "重击频率0.1，元素比例0.9，倍率2.57",
            "E频率0.2，倍率3.276",
            "Q频率0.1，倍率6.558",
        ]
    }
]