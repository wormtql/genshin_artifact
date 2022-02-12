import card from "./card.jpg"
import avatar from "@asset/badges/tuoma.png"
import splash from "./splash.png"

const g1 = "普通攻击·迅破枪势"
const g2 = "烈烧佑命之侍护"
const g3 = "真红炽火之大铠"

export default {
    card,
    name: "Thoma",
    chs: "托马",
    element: "Pyro",
    weapon: "Polearm",
    star: 4,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害/2", group: g1 },
        { index: 4, chs: "四段伤害", group: g1 },
        { index: 5, chs: "重击伤害", group: g1 },
        { index: 6, chs: "下坠期间伤害", group: g1 },
        { index: 7, chs: "低空坠地冲击伤害", group: g1 },
        { index: 8, chs: "高空坠地冲击伤害", group: g1 },
        { index: 9, chs: "技能伤害", group: g2 },
        { index: 10, chs: "技能伤害", group: g3 },
        { index: 11, chs: "炽火崩破伤害", group: g3 },
    ]
}
