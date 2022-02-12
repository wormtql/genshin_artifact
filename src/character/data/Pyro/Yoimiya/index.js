import card from "./card.png"
import avatar from "@asset/badges/xiaogong.png"
import splash from "./splash.png"

const g1 = "普通攻击·烟火打扬"
const g2 = "焰硝庭火舞"
const g3 = "琉金云间草"

export default {
    card,
    name: "Yoimiya",
    chs: "宵宫",
    element: "Pyro",
    weapon: "Bow",
    star: 5,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害/2", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害", group: g1 },
        { index: 3, chs: "四段伤害/2", group: g1 },
        { index: 4, chs: "五段伤害", group: g1 },
        { index: 5, chs: "瞄准射击", group: g1 },
        { index: 6, chs: "满蓄力瞄准射击", group: g1 },
        { index: 7, chs: "焰硝矢伤害", group: g1 },
        { index: 8, chs: "下坠期间伤害", group: g1 },
        { index: 9, chs: "低空坠地冲击伤害", group: g1 },
        { index: 10, chs: "高空坠地冲击伤害", group: g1 },
        { index: 11, chs: "技能伤害", group: g3 },
        { index: 12, chs: "琉金火光爆炸伤害", group: g3 },
    ]
}
