function normalQiqi(attribute) {
    const qRatio = 0.5;

    let attack = attribute.attack1 + attribute.attack2;
    let a = attack * 0.1584 + 116;
    let b = attack * 1.35 + 996;
    let ab = qRatio * a + (1 - qRatio) * b;
    
    return {
        value: ab * (1 + attribute.cureEffect)
    }
}

export const plans = [
    {
        value: "qiqi_normal",
        name: "常规七七",
        calc: normalQiqi,
        description: "常规七七，目标：对队友的期望治疗量（自己无论如何都不太会容易残血）",
        tags: [
            "Q、E平分",
        ]
    }
]