import card from "./card.jpg"
import avatar from "@asset/badges/babala.png"
import splash from "./splash.png"

const g1 = "普通攻击·水之浅唱"
const g2 = "演唱，开始♪"
const g3 = "闪耀奇迹♪"

export default {
    name: "Barbara",
    chs: "芭芭拉",
    element: "Hydro",
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
        { index: 8, chs: "命中治疗量", group: g2 },
        { index: 9, chs: "水珠伤害", group: g2 },
        { index: 10, chs: "持续治疗量", group: g2 },
        { index: 11, chs: "治疗量", group: g3 }
    ]
}