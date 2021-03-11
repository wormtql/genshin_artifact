import createFreqBasedDamageFunction from "../../factory/freq_based_func";

let args = {
    element: "fire",

    aFreq: 55,
    aRatio: 0.7,
    aTimes: 1.49,

    bFreq: 0,
    bRatio: 0,
    bTimes: 0,

    eFreq: 40,
    eTimes: 1.497,

    qFreq: 5,
    qTimes: 5.38,
}

export default {
    name: "dilukeNormal",
    func: createFreqBasedDamageFunction(args),
}