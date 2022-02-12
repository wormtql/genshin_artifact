import card from "./card.jpg"
import avatar from "@asset/badges/nuoaier.png"
import splash from "./splash.png"

const g1 = "普通攻击·西风剑术·女仆"
const g2 = "护心铠"
const g3 = "大扫除"

export default {
    name: "Noelle",
    chs: "诺艾尔",
    element: "Geo",
    weapon: "Claymore",
    star: 4,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "重击循环伤害", group: g1 },
        { index: 5, chs: "重击终结伤害", group: g1 },
        { index: 6, chs: "下坠期间伤害", group: g1 },
        { index: 7, chs: "低空坠地冲击伤害", group: g1 },
        { index: 8, chs: "高空坠地冲击伤害", group: g1 },
        { index: 9, chs: "技能伤害", group: g2 },
        { index: 10, chs: "治疗量", group: g2 },
        { index: 11, chs: "爆发伤害", group: g3 },
        { index: 12, chs: "技能伤害", group: g3 },
    ]
}