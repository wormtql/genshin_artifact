import card from "./card.jpg"
import avatar from "@asset/badges/abeiduo.png"
// import skill from "./abeiduo.skill.js"
import splash from "./splash.png"

const g1 = "普通攻击·西风剑术·白"
const g2 = "创生法·拟造阳华"
const g3 = "诞生式·大地之潮"

export default {
    name: "Albedo",
    chs: "阿贝多",
    element: "Geo",
    weapon: "Sword",
    star: 5,
    card,
    avatar,
    splash,
    // skill,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害", group: g1 },
        { index: 5, chs: "重击伤害", group: g1 },
        { index: 6, chs: "下坠期间伤害", group: g1 },
        { index: 7, chs: "低空坠地冲击伤害", group: g1 },
        { index: 8, chs: "高空坠地冲击伤害", group: g1 },
        { index: 9, chs: "技能伤害", group: g2 },
        { index: 10, chs: "刹那之花伤害", group: g2 },
        { index: 11, chs: "爆发伤害", group: g3 },
        { index: 12, chs: "生灭之花", group: g3 },
    ]
}