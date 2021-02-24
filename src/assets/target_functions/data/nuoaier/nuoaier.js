import badge from "./badge.png";

function normalNuoaier(attribute) { 
    let def = attribute.defend();
    let atk = attribute.attack() + def * 0.56;

    let crit = Math.min(1, attribute.critical);

    let bonus = attribute.bonus + attribute.rockBonus + attribute.aBonus;
    
    let dmg = 1.28 * atk * (1 + bonus) * (1 + crit * attribute.criticalDamage);
    return dmg;
}

export default {
    name: "nuoaierNormal",
    chs: "诺艾尔-原石的重量，令人安息",
    description: [
        "使得诺艾尔大招伤害最高",
    ],
    tags: [
        "输出",
        "诺艾尔",
    ],
    func: normalNuoaier,
    "for": "nuoaier",
    badge,
}