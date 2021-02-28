import badge from "./badges/fire_slime.png";

import { charactersData } from "@asset/characters";

function maxQ(config) {
    let characterData = charactersData[config.character.name];
    let element = characterData.element;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute.qBonus;

        let baseDmg = atk * (1 + bonus);
        return (1 + attribute.criticalDamage) * baseDmg;
    };
}

export default {
    name: "maxQ",
    chs: "上限-元素伤害（Q技能）",
    description: [
        "相当于使得Q技能伤害上限最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素伤害加成 + Q伤害加成) * (1 + 暴击伤害)",
    tags: [
        "上限",
        "元素",
    ],
    func: maxQ,
    "for": "common",
    badge,
    needConfig: true,
}