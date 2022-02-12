import card from "./card.jpg"
import avatar from "@asset/badges/diluke.png"
import splash from "./splash.png"

const g1 = "普通攻击·淬炼之剑"
const g2 = "逆焰之刃"
const g3 = "黎明"

export default {
    name: "Diluc",
    chs: "迪卢克",
    element: "Pyro",
    weapon: "Claymore",
    star: 5,
    avatar,
    card,
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
        { index: 9, chs: "一段伤害", group: g2 },
        { index: 10, chs: "二段伤害", group: g2 },
        { index: 11, chs: "三段伤害", group: g2 },
        { index: 12, chs: "斩击伤害", group: g3 },
        { index: 13, chs: "持续伤害", group: g3 },
        { index: 14, chs: "爆裂伤害", group: g3 }
    ]
}