import card from "./card.jpg"
import avatar from "@asset/badges/ningguang.png"
import splash from "./splash.png"

const g1 = "普通攻击·千金掷"
const g2 = "璇玑屏"
const g3 = "天权崩玉"

export default {
    name: "Ningguang",
    chs: "凝光",
    element: "Geo",
    weapon: "Catalyst",
    star: 4,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "普通攻击伤害", group: g1 },
        { index: 1, chs: "重击伤害", group: g1 },
        { index: 2, chs: "星璇伤害", group: g1 },
        { index: 3, chs: "下坠期间伤害", group: g1 },
        { index: 4, chs: "低空坠地冲击伤害", group: g1 },
        { index: 5, chs: "高空坠地冲击伤害", group: g1 },
        { index: 6, chs: "技能伤害", group: g2 },
        { index: 7, chs: "每颗宝石伤害", group: g3 }
    ]
}