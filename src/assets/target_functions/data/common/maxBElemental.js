import badge from "./badges/fire_slime.png";

import { charactersData } from "@asset/characters";

function maxB(config) {
    let characterData = charactersData[config.character.name];
    let element = characterData.element;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute.bBonus;

        let baseDmg = atk * (1 + bonus);
        return (attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "maxBElemental",
    chs: "上限-元素伤害（重击）",
    description: [
        "使得重击元素伤害的上限最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素伤害加成 + 重击伤害加成) * (1 + 暴击伤害)",
    tags: [
        "上限",
        "元素",
    ],
    func: maxB,
    "for": "common",
    badge,
    needConfig: true,
}