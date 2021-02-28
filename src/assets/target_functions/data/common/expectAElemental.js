import badge from "./badges/thunder_slime.png";

import { charactersData } from "@asset/characters";

function expectA(config) {
    let characterData = charactersData[config.character.name];
    let element = characterData.element;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute.aBonus;
        let crit = Math.min(attribute.critical, 1);

        let baseDmg = atk * (1 + bonus);
        return (crit * attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "expectAElemental",
    chs: "期望-元素伤害（平A）",
    description: [
        "使得平A元素伤害的期望伤害最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素伤害加成 + 平A伤害加成) * (1 + 平A暴击率 * 暴击伤害)",
    tags: [
        "期望",
        "元素",
    ],
    func: expectA,
    "for": "common",
    badge,
    needConfig: true,
}