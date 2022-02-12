import card from "./card.png"
import avatar from "@asset/badges/fengyuanwanye.png"
// import skill from "./fengyuanwanye.skill.js";
import splash from "./splash.png"

const g1 = "普通攻击·我流剑术"
const gPlunge = "下落攻击•乱岚拨止"
const g2 = "千早振"
const g3 = "万叶之一刀"

export default {
    name: "KaedeharaKazuha",
    chs: "枫原万叶",
    element: "Anemo",
    weapon: "Sword",
    star: 5,
    card,
    avatar,
    splash,
    skillMap: [
        { index: 0, chs: "一段伤害", group: g1 },
        { index: 0, chs: "二段伤害", group: g1 },
        { index: 0, chs: "三段伤害-1", group: g1 },
        { index: 0, chs: "三段伤害-2", group: g1 },
        { index: 0, chs: "四段伤害", group: g1 },
        { index: 0, chs: "五段伤害/3", group: g1 },
        { index: 0, chs: "重击伤害-1", group: g1 },
        { index: 0, chs: "重击伤害-2", group: g1 },
        { index: 0, chs: "下坠期间伤害", group: g1 },
        { index: 0, chs: "低空坠地冲击伤害", group: g1 },
        { index: 0, chs: "高空坠地冲击伤害", group: g1 },
        { index: 0, chs: "乱岚拨止：下坠期间伤害", group: gPlunge },
        { index: 0, chs: "乱岚拨止：低空坠地冲击伤害", group: gPlunge },
        { index: 0, chs: "乱岚拨止：高空坠地冲击伤害", group: gPlunge },
        { index: 0, chs: "乱岚拨止：火元素转化伤害", group: gPlunge },
        { index: 0, chs: "乱岚拨止：水元素转化伤害", group: gPlunge },
        { index: 0, chs: "乱岚拨止：冰元素转化伤害", group: gPlunge },
        { index: 0, chs: "乱岚拨止：雷元素转化伤害", group: gPlunge },
        { index: 0, chs: "点按技能伤害", group: g2 },
        { index: 0, chs: "长按技能伤害", group: g2 },
        { index: 0, chs: "斩击伤害", group: g3 },
        { index: 0, chs: "持续伤害", group: g3 },
        { index: 0, chs: "附加火元素伤害", group: g3 },
        { index: 0, chs: "附加水元素伤害", group: g3 },
        { index: 0, chs: "附加冰元素伤害", group: g3 },
        { index: 0, chs: "附加雷元素伤害", group: g3 }
    ]
    // skill,
}