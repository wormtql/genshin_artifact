import card from "./card.png"
import avatar from "@asset/badges/youla.png"
import splash from "./splash.png"

const g1 = "普通攻击·西风剑术·宗室"
const g2 = "冰潮的涡旋"
const g3 = "凝浪之光剑"

export default {
    name: "Eula",
    chs: "优菈",
    element: "Cryo",
    weapon: "Claymore",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害-1", group: g1 },
        { index: 3, chs: "三段伤害-2", group: g1 },
        { index: 4, chs: "四段伤害", group: g1 },
        { index: 5, chs: "五段伤害-1", group: g1 },
        { index: 6, chs: "五段伤害-2", group: g1 },
        { index: 7, chs: "重击循环伤害", group: g1 },
        { index: 8, chs: "重击终结伤害", group: g1 },
        { index: 9, chs: "下坠期间伤害", group: g1 },
        { index: 10, chs: "低空坠地冲击伤害", group: g1 },
        { index: 11, chs: "高空坠地冲击伤害", group: g1 },
        { index: 12, chs: "点按伤害", group: g2 },
        { index: 13, chs: "长按伤害", group: g2 },
        { index: 14, chs: "冰涡之剑伤害", group: g2 },
        { index: 15, chs: "残缺光降之剑", group: g2 },
        { index: 16, chs: "技能伤害", group: g3 },
        { index: 17, chs: "光降之剑", group: g3 }
    ]
}