import card from "./card.jpg"
import avatar from "@asset/badges/hutao.png"
import splash from "./splash.png"

const g1 = "普通攻击·往生秘传枪法"
const g2 = "蝶引来生"
const g3 = "安神秘法"

export default {
    name: "HuTao",
    chs: "胡桃",
    element: "Pyro",
    weapon: "Polearm",
    star: 5,
    avatar,
    card,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害", group: g1 },
        { index: 4, chs: "五段伤害-1", group: g1 },
        { index: 5, chs: "五段伤害-2", group: g1 },
        { index: 6, chs: "六段伤害", group: g1 },
        { index: 7, chs: "重击伤害", group: g1 },
        { index: 8, chs: "下坠期间伤害", group: g1 },
        { index: 9, chs: "低空坠地冲击伤害", group: g1 },
        { index: 10, chs: "高空坠地冲击伤害", group: g1 },
        { index: 11, chs: "血梅香伤害", group: g2 },
        { index: 12, chs: "技能伤害", group: g3 },
        { index: 13, chs: "低血量时技能伤害", group: g3 },
    ]
}