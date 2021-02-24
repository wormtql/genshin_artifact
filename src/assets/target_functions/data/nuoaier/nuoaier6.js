import badge from "./badge.png";

function normal6Nuoaier(attribute) { 
    let def = attribute.defend();
    let atk = attribute.attack() + def * 1.06;

    let crit = Math.min(1, attribute.critical);

    let bonus = attribute.bonus + attribute.rockBonus + attribute.aBonus;
    
    let dmg = 0.128 * atk * (1 + bonus) * (1 + crit * attribute.criticalDamage);
    return dmg;
}

export default {
    name: "nuoaier6Normal",
    chs: "诺艾尔-原石的重量，令人安息-6命限定",
    description: [
        "使得诺艾尔大招伤害最高",
        "需要6命诺艾尔"
    ],
    tags: [
        "输出",
        "诺艾尔",
    ],
    func: normal6Nuoaier,
    "for": "nuoaier",
    badge,
}