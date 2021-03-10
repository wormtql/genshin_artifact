import badge from "@asset/badges/fire_slime.png";
import config from "./maxConfig";


function f(config) {
    let element = config.tArgs.element;
    let skill = config.tArgs.skill;

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute[skill + "Bonus"];

        let baseDmg = atk * (1 + bonus);
        return (attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "max",
    chs: "最大伤害",
    description: [
        "使得某种类型的伤害上限最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素/物理伤害加成 + 平A/重击/元素战技/元素爆发伤害加成) * (1 + 暴击伤害)",
    tags: [
        "上限",
    ],
    func: f,
    "for": "common",
    badge,
    needConfig: true,
    config,
}