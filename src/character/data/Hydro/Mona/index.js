import card from "./card.jpg"
import avatar from "@asset/badges/mona.png"
import splash from "./splash.png"

const g1 = "普通攻击·因果点破"
const g2 = "水中幻愿"
const g3 = "虚实流动"

export default {
    name: "Mona",
    chs: "莫娜",
    element: "Hydro",
    weapon: "Catalyst",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "重击伤害", group: g1 },
        { index: 5, chs: "下坠期间伤害", group: g1 },
        { index: 6, chs: "低空坠地冲击伤害", group: g1 },
        { index: 7, chs: "高空坠地冲击伤害", group: g1 },
        { index: 8, chs: "持续伤害", group: g2 },
        { index: 9, chs: "爆裂伤害", group: g2 },
        { index: 10, chs: "泡影破裂伤害", group: g3 },
    ]
}
