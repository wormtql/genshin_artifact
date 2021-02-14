import badge from "./badge.png";

function normalQiqi(attribute) {
    const qRatio = 0.5;

    let attack = attribute.attack();
    let a = attack * 0.1584 + 116;
    let b = attack * 1.35 + 996;
    let ab = qRatio * a + (1 - qRatio) * b;
    
    return ab * (1 + attribute.cureEffect)
}

export default {
    name: "qiqiNormal",
    chs: "七七-冻冻回魂夜",
    description: [
        "使得七七总的治疗效果最好",
    ],
    tags: [
        "治疗",
        "七七",
    ],
    func: normalQiqi,
    "for": "qiqi",
    badge,
}