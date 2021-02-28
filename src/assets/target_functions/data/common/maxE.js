import badge from "./badges/fire_slime.png";

import { charactersData } from "@asset/characters";

function maxE(config) {
    let characterData = charactersData[config.character.name];
    let element = characterData.element;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute.eBonus;

        let baseDmg = atk * (1 + bonus);
        return (1 + attribute.criticalDamage) * baseDmg;
    };
}

export default {
    name: "maxE",
    chs: "上限-元素伤害（E技能）",
    description: [
        "相当于使得E技能伤害上限最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素伤害加成 + E伤害加成) * (1 + 暴击伤害)",
    tags: [
        "上限",
        "元素",
    ],
    func: maxE,
    "for": "common",
    badge,
    needConfig: true,
}