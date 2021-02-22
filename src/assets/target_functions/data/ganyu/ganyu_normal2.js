import badge from "./badge.png";

function ganyuNormal2(attribute) {
    let attack = attribute.attack();

    let bonus = attribute.bBonus + attribute.bonus + attribute.iceBonus;
    let baseDmg = attack * (1 + bonus);

    return baseDmg * (1 + attribute.criticalDamage);
}

export default {
    name: "ganyuNormal2",
    chs: "甘雨-王小美",
    description: [
        "靠二段蓄力输出",
        "不考虑暴击率",
        "最好带了阿莫斯之弓以及限定了4冰套",
    ],
    tags: [
        "输出",
        "甘雨",
    ],
    func: ganyuNormal2,
    "for": "ganyu",
    badge,
}