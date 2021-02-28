import badge from "./badges/thunder_slime.png";

import { charactersData } from "@asset/characters";

function expectQ(config) {
    let characterData = charactersData[config.character.name];
    let element = characterData.element;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute.qBonus;
        let crit = Math.min(attribute.qCritical, 1);

        let baseDmg = atk * (1 + bonus);
        return (crit * attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "expectQ",
    chs: "期望-元素伤害（Q技能）",
    description: [
        "使得Q技能期望伤害最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素伤害加成 + Q伤害加成) * (1 + Q暴击率 * 暴击伤害)",
    tags: [
        "期望",
        "元素",
    ],
    func: expectQ,
    "for": "common",
    badge,
    needConfig: true,
}