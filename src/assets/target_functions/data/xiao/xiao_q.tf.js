import badge from "@asset/badges/xiao.png";

function xiaoQ(attribute) {
    let attack = attribute.attack();
    let crit = Math.min(attribute.airCritical, 1);

    let bonus = attribute.airBonus + attribute.bonus + attribute.windBonus;
    let baseDmg = attack * (1 + bonus);

    return (crit * attribute.criticalDamage + 1) * baseDmg;
}

export default {
    name: "xiaoQ",
    chs: "魈-靖妖傩舞",
    description: [
        "使得开大后的下落攻击期望伤害最高",
    ],
    tags: [
        "输出",
        "魈",
    ],
    func: xiaoQ,
    "for": "xiao",
    badge,
}