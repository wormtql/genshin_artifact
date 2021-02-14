import badge from "./badges/sword.png";
import expectFunction from "../factory/expect";

export default {
    name: "physicalExpect",
    chs: "期望-物理伤害",
    description: [
        "相当于使得多次平A的伤害平均值最高",
    ],
    tags: [
        "攻击",
        "物理",
    ],
    func: expectFunction("physical"),
    "for": "common",
    badge,
}