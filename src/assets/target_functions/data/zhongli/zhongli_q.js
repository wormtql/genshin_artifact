import badge from "./badge.png";

function zhongliQ(attribute) {

    let attack = attribute.attack();
    let bonus = attribute.bonus + attribute.rockBonus + attribute.qBonus;
    let extraDmg = attribute.life() * 0.33;
    let baseDmg = 6.4 * attack * (1 + bonus) + extraDmg;

    let critDmg = (1 + attribute.criticalDamage) * baseDmg;
    return critDmg;
}

export default {
    name: "zhongliQ",
    chs: "钟离-天动万象",
    description: [
        "使得钟离Q技能的伤害最大值最高（在天赋：炊金馔玉的加成下）",
        "假设技能6级（不同技能等级差别不大）",
    ],
    tags: [
        "钟离",
        "赌狗",
    ],
    func: zhongliQ,
    "for": "zhongli",
    badge,
}