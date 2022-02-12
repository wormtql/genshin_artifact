import card from "./card.jpg"
import avatar from "@asset/badges/kaiya.png"
// import skill from "./kaiya.skill.js";
import splash from "./splash.png"

const g1 = "普通攻击·仪典剑术"
const g2 = "霜袭"
const g3 = "凛冽轮舞"

export default {
    name: "Kaeya",
    chs: "凯亚",
    element: "Cryo",
    weapon: "Sword",
    star: 4,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害", group: g1 },
        { index: 5, chs: "重击伤害-1", group: g1 },
        { index: 6, chs: "重击伤害-2", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "技能伤害", group: g2 },
        { index: 11, chs: "技能伤害", group: g3 }
    ]
}