import badge from "@asset/badges/fire_slime.png";
import config from "./expectConfig";


function f(config) {
    let element = config.tArgs.element;
    let skill = config.tArgs.skill;
    let critName = skill === "a" ? "critical" : (skill + "Critical");

    return function(attribute) {
        let atk = attribute.attack();
        let bonus = attribute.bonus + attribute[element + "Bonus"] + attribute[skill + "Bonus"];

        let crit = Math.min(attribute[critName], 1);

        let baseDmg = atk * (1 + bonus);
        return (crit * attribute.criticalDamage + 1) * baseDmg;
    };
}

export default {
    name: "expect",
    chs: "期望伤害",
    description: [
        "使得某种类型的期望伤害最高"
    ],
    formula: "攻击力 * (1 + 伤害加成 + 元素/物理伤害加成 + 平A/重击/元素战技/元素爆发伤害加成) * (1 + 相应技能暴击率 * 暴击伤害)",
    tags: [
        "期望",
    ],
    func: f,
    "for": "common",
    badge,
    needConfig: true,
    config,
}