import badge from "@asset/badges/fire_slime.png";
import config from "./ExpectConfig";

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
    "for": "common",
    badge,
    config,
}