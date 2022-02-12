import card from "./card.jpg"
import avatar from "@asset/badges/keli.png"
import splash from "./splash.png"

const g1 = "普通攻击·砰砰"
const g2 = "蹦蹦炸弹"
const g3 = "轰轰火花"

export default {
    name: "Klee",
    chs: "可莉",
    element: "Pyro",
    weapon: "Catalyst",
    star: 5,
    avatar,
    card,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "重击伤害", group: g1 },
        { index: 4, chs: "下坠期间伤害", group: g1 },
        { index: 5, chs: "低空坠地冲击伤害", group: g1 },
        { index: 6, chs: "高空坠地冲击伤害", group: g1 },
        { index: 7, chs: "蹦蹦炸弹伤害", group: g2 },
        { index: 8, chs: "诡雷伤害", group: g2 },
        { index: 9, chs: "轰轰火花伤害", group: g3 }
    ]
}
