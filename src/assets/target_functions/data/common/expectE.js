import badge from "./badges/thunder_slime.png";

import { charactersData } from "@asset/characters";

function expectE(config) {
    let characterData = charactersData[config.character.name];
    let element = characterData.element;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute.eBonus;
        let crit = Math.min(attribute.eCritical, 1);

        let baseDmg = atk * (1 + bonus);
        return (crit * attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "expectE",
    chs: "期望-元素伤害（E技能）",
    description: [
        "使得E技能期望伤害最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素伤害加成 + E伤害加成) * (1 + E暴击率 * 暴击伤害)",
    tags: [
        "期望",
        "元素",
    ],
    func: expectE,
    "for": "common",
    badge,
    needConfig: true,
}