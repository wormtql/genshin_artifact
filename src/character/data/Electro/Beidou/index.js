import card from "./card.jpg"
import avatar from "@asset/badges/beidou.png"
import splash from "./splash.png"

const g1 = "普通攻击·征涛"
const g2 = "捉浪"
const g3 = "斫雷"

export default {
    name: "Beidou",
    chs: "北斗",
    element: "Electro",
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
        { index: 4, chs: "五段伤害", group: g1 },
        { index: 5, chs: "重击循环伤害", group: g1 },
        { index: 6, chs: "重击终结伤害", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "基础伤害", group: g2 },
        { index: 11, chs: "一层伤害", group: g2 },
        { index: 12, chs: "二层伤害", group: g2 },
        { index: 13, chs: "技能伤害", group: g3 },
        { index: 14, chs: "闪电伤害", group: g3 }
    ]
}
