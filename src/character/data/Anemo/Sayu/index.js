import card from "./card.png"
import avatar from "@asset/badges/zaoyou.png"
import splash from "./splash.png"

const g1 = "普通攻击·忍刀·终末番"
const g2 = "呜呼流·风隐急进"
const g3 = "呜呼流·影貉缭乱"

export default {
    name: "Sayu",
    chs: "早柚",
    element: "Anemo",
    weapon: "Claymore",
    star: 4,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 1, chs: "二段伤害", group: g1 },
        { index: 2, chs: "三段伤害-1", group: g1 },
        { index: 3, chs: "三段伤害-2", group: g1 },
        { index: 4, chs: "四段伤害", group: g1 },
        { index: 5, chs: "重击循环伤害", group: g1 },
        { index: 6, chs: "重击终结伤害", group: g1 },
        { index: 7, chs: "下坠期间伤害", group: g1 },
        { index: 8, chs: "低空坠地冲击伤害", group: g1 },
        { index: 9, chs: "高空坠地冲击伤害", group: g1 },
        { index: 10, chs: "风风轮伤害", group: g2 },
        { index: 11, chs: "风风轮舞踢点按伤害", group: g2 },
        { index: 12, chs: "风风轮舞踢长按伤害", group: g2 },
        { index: 13, chs: "风风轮附带火元素伤害", group: g2 },
        { index: 14, chs: "风风轮附带雷元素伤害", group: g2 },
        { index: 15, chs: "风风轮附带冰元素伤害", group: g2 },
        { index: 16, chs: "风风轮附带水元素伤害", group: g2 },
        { index: 17, chs: "风风轮舞踢长按附带火元素伤害", group: g2 },
        { index: 18, chs: "风风轮舞踢长按附带雷元素伤害", group: g2 },
        { index: 19, chs: "风风轮舞踢长按附带冰元素伤害", group: g2 },
        { index: 20, chs: "风风轮舞踢长按附带水元素伤害", group: g2 },
        { index: 21, chs: "技能发动伤害", group: g3 },
        { index: 22, chs: "技能发动治疗量", group: g3 },
        { index: 23, chs: "不倒貉貉伤害", group: g3 },
        { index: 24, chs: "不倒貉貉治疗量", group: g3 },
    ]
}
