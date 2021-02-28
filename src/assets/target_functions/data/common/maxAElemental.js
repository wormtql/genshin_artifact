import badge from "./badges/fire_slime.png";

import { charactersData } from "@asset/characters";

function maxA(config) {
    let characterData = charactersData[config.character.name];
    let element = characterData.element;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute.aBonus;

        let baseDmg = atk * (1 + bonus);
        return (attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "maxAElemental",
    chs: "上限-元素伤害（平A）",
    description: [
        "使得平A元素伤害的上限最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素伤害加成 + 平A伤害加成) * (1 + 暴击伤害)",
    tags: [
        "上限",
        "元素",
    ],
    func: maxA,
    "for": "common",
    badge,
    needConfig: true,
}