import card from "./card.jpg"
import avatar from "@asset/badges/xingqiu.png"
import splash from "./splash.png"

const g1 = "普通攻击·古华剑法"
const g2 = "古华剑·画雨笼山"
const g3 = "古华剑·裁雨留虹"

export default {
    name: "Xingqiu",
    chs: "行秋",
    element: "Hydro",
    weapon: "Sword",
    star: 4,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害-1", group: g1 },
        { index: 3, chs: "三段伤害-2", group: g1 },
        { index: 5, chs: "四段伤害", group: g1 },
        { index: 6, chs: "五段伤害-1", group: g1 },
        { index: 7, chs: "五段伤害-2", group: g1 },
        { index: 9, chs: "重击伤害-1", group: g1 },
        { index: 10, chs: "重击伤害-2", group: g1 },
        { index: 12, chs: "下坠期间伤害", group: g1 },
        { index: 13, chs: "低空坠地冲击伤害", group: g1 },
        { index: 14, chs: "高空坠地冲击伤害", group: g1 },
        { index: 15, chs: "技能伤害-1", group: g2 },
        { index: 16, chs: "技能伤害-2", group: g2 },
        { index: 17, chs: "剑雨伤害", group: g3 },
    ]
}
