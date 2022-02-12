import card from "./card.jpg"
import avatar from "@asset/badges/qiqi.png"
import splash from "./splash.png"

const g1 = "普通攻击·云来古剑法"
const g2 = "仙法·寒病鬼差"
const g3 = "仙法·救苦度厄"

export default {
    name: "Qiqi",
    chs: "七七",
    element: "Cryo",
    weapon: "Sword",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害-1", group: g1 },
        { index: 3, chs: "三段伤害-2", group: g1 },
        { index: 4, chs: "四段伤害-1", group: g1 },
        { index: 5, chs: "四段伤害-2", group: g1 },
        { index: 6, chs: "五段伤害", group: g1 },
        { index: 7, chs: "重击伤害-1", group: g1 },
        { index: 8, chs: "重击伤害-2", group: g1 },
        { index: 9, chs: "下坠期间伤害", group: g1 },
        { index: 10, chs: "低空坠地冲击伤害", group: g1 },
        { index: 11, chs: "高空坠地冲击伤害", group: g1 },
        { index: 12, chs: "技能伤害", group: g2 },
        { index: 13, chs: "命中治疗量", group: g2 },
        { index: 14, chs: "持续治疗量", group: g2 },
        { index: 15, chs: "寒病鬼差伤害", group: g2 },
        { index: 16, chs: "技能伤害", group: g3 },
        { index: 17, chs: "治疗量", group: g3 },
    ]
}