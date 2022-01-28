import card from "./card.jpg"
import avatar from "@asset/badges/diaona.png"
// import skill from "./diaona.skill.js";
import splash from "./splash.png"

const g1 = "普通攻击·猎人射术"
const g2 = "猫爪冻冻"
const g3 = "最烈特调"

export default {
    name: "Diona",
    chs: "迪奥娜",
    element: "Cryo",
    weapon: "Bow",
    star: 4,
    avatar,
    card,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害", group: g1 },
        { index: 5, chs: "瞄准射击", group: g1 },
        { index: 6, chs: "满蓄力瞄准射击", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "猫爪伤害", group: g2 },
        { index: 11, chs: "技能伤害", group: g3 },
        { index: 12, chs: "领域持续伤害", group: g3 },
        { index: 13, chs: "持续治疗量", group: g3 }
    ]
    // skill,
}
