import card from "./card.jpg"
import avatar from "@asset/badges/yanfei.png"
import splash from "./splash.png"

const g1 = "普通攻击·火漆制印"
const g2 = "丹书立约"
const g3 = "凭此结契"

export default {
    name: "Yanfei",
    chs: "烟绯",
    element: "Pyro",
    weapon: "Catalyst",
    star: 4,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "三段伤害", group: g1 },
        { index: 4, chs: "重击-无印伤害", group: g1 },
        { index: 5, chs: "重击-1层伤害", group: g1 },
        { index: 6, chs: "重击-2层伤害", group: g1 },
        { index: 7, chs: "重击-3层伤害", group: g1 },
        { index: 8, chs: "重击-4层伤害", group: g1 },
        { index: 9, chs: "天赋2额外伤害", group: g1 },
        { index: 10, chs: "下坠期间伤害", group: g1 },
        { index: 11, chs: "低空坠地冲击伤害", group: g1 },
        { index: 12, chs: "高空坠地冲击伤害", group: g1 },
        { index: 13, chs: "技能伤害", group: g2 },
        { index: 14, chs: "技能伤害", group: g3 },
    ]
}
