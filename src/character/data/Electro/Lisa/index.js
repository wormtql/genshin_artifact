import card from "./card.jpg"
import avatar from "@asset/badges/lisha.png"
import splash from "./splash.png"

const g1 = "普通攻击·指尖雷暴"
const g2 = "苍雷"
const g3 = "蔷薇的雷光"

export default {
    name: "Lisa",
    chs: "丽莎",
    element: "Electro",
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
        { index: 8, chs: "点按伤害", group: g2 },
        { index: 9, chs: "无引雷长按伤害", group: g2 },
        { index: 10, chs: "一层引雷长按伤害", group: g2 },
        { index: 11, chs: "二层引雷长按伤害", group: g2 },
        { index: 12, chs: "三层引雷长按伤害", group: g2 },
        { index: 13, chs: "雷光放电伤害", group: g3 }
    ]
}
