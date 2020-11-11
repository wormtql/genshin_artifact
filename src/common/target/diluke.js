import { all } from "./base";

let calc = all({
    element: "fire",
    aFreq: 30,
    aRatio: 0.3,
    aTimes: 1.1,
    bFreq: 0,
    bRatio: 0,
    bTimes: 0,
    eFreq: 50,
    eTimes: 1.603,
    qFreq: 10,
    qTimes: 7.75
})

export const plans = [
    {
        value: "diluke-normal",
        name: "常规迪卢克",
        calc,
        description: "普通的输出型迪卢克",
        tags: [
            "普攻频率0.3，元素比例0.3，倍率1.1",
            "重击频率0",
            "E频率0.5，倍率1.603",
            "Q频率0.1，倍率7.75",
        ]
    }
]