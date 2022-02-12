import card from "./card.jpg"
import avatar from "@asset/badges/shatang.png"
import splash from "./splash.png"

const g1 = "普通攻击·简式风灵作成"
const g2 = "风灵作成·陆叁零捌"
const g3 = "禁·风灵作成·柒伍同构贰型"

export default {
    name: "Sucrose",
    chs: "砂糖",
    element: "Anemo",
    weapon: "Catalyst",
    star: 4,
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
        { index: 8, chs: "技能伤害", group: g2 },
        { index: 9, chs: "持续伤害", group: g3 },
        { index: 10, chs: "附加火元素伤害", group: g3 },
        { index: 11, chs: "附加水元素伤害", group: g3 },
        { index: 12, chs: "附加雷元素伤害", group: g3 },
        { index: 13, chs: "附加冰元素伤害", group: g3 },
    ]
}
