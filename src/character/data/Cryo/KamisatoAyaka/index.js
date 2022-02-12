import card from "./card.png"
import avatar from "@asset/badges/shenlilinghua.png"
import splash from "./splash.png"

const g1 = "普通攻击·神里流·倾"
const g2 = "神里流·冰华"
const g3 = "神里流·霜灭"

export default {
    name: "KamisatoAyaka",
    chs: "神里绫华",
    element: "Cryo",
    weapon: "Sword",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 4, chs: "四段伤害", group: g1 },
        { index: 5, chs: "五段伤害", group: g1 },
        { index: 6, chs: "重击伤害/3", group: g1 },
        { index: 8, chs: "下坠期间伤害", group: g1 },
        { index: 9, chs: "低空坠地冲击伤害", group: g1 },
        { index: 10, chs: "高空坠地冲击伤害", group: g1 },
        { index: 11, chs: "技能伤害", group: g2 },
        { index: 12, chs: "切割伤害", group: g3 },
        { index: 13, chs: "绽放伤害", group: g3 }
    ]
}