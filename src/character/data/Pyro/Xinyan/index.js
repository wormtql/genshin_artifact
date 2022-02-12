import card from "./card.jpg"
import avatar from "@asset/badges/xinyan.png"
import splash from "./splash.png"

const g1 = "普通攻击·炎舞"
const g2 = "热情拂扫"
const g3 = "叛逆刮弦"

export default {
    name: "Xinyan",
    chs: "辛焱",
    element: "Pyro",
    weapon: "Claymore",
    star: 4,
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
        { index: 9, chs: "挥舞伤害", group: g2 },
        { index: 10, chs: "持续伤害", group: g2 },
        { index: 11, chs: "技能伤害", group: g3 },
        { index: 12, chs: "火元素持续伤害", group: g3 }
    ]
}
