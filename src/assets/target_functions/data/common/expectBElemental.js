import badge from "./badges/thunder_slime.png";

import { charactersData } from "@asset/characters";

function expectB(config) {
    let characterData = charactersData[config.character.name];
    let element = characterData.element;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute.bBonus;
        let crit = Math.min(attribute.bCritical, 1);

        let baseDmg = atk * (1 + bonus);
        return (crit * attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "expectBElemental",
    chs: "期望-元素伤害（重击）",
    description: [
        "使得重击元素伤害的期望伤害最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素伤害加成 + 重击伤害加成) * (1 + 重击暴击率 * 暴击伤害)",
    tags: [
        "期望",
        "元素",
    ],
    func: expectB,
    "for": "common",
    badge,
    needConfig: true,
}