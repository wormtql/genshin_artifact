import badge from "@asset/badges/fire_slime.png";
import config from "./MaxTargetFuncConfig";

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
    "for": "common",
    config,
    badge,
}