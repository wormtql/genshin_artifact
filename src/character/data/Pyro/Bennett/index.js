import card from "./card.jpg"
import avatar from "@asset/badges/bannite.png"
import splash from "./splash.png"

const g1 = "普通攻击·好运剑"
const g2 = "热情过载"
const g3 = "美妙旅程"

export default {
    card,
    name: "Bennett",
    chs: "班尼特",
    element: "Pyro",
    weapon: "Sword",
    star: 4,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害", group: g1 },
        { index: 5, chs: "重击伤害-1", group: g1 },
        { index: 6, chs: "重击伤害-2", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "点按伤害", group: g2 },
        { index: 11, chs: "一段蓄力伤害-1", group: g2 },
        { index: 12, chs: "一段蓄力伤害-2", group: g2 },
        { index: 13, chs: "二段蓄力伤害-1", group: g2 },
        { index: 14, chs: "二段蓄力伤害-2", group: g2 },
        { index: 15, chs: "爆炸伤害", group: g2 },
        { index: 16, chs: "技能伤害", group: g3 },
        { index: 17, chs: "持续治疗", group: g3 }
    ]
}