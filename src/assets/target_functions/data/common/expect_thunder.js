import badge from "./badges/thunder_slime.png";
import expectFunction from "../factory/expect";

export default {
    name: "thunderExpect",
    chs: "期望-雷元素伤害",
    description: [
        "相当于使得多次E（雷）的伤害平均值最高",
    ],
    tags: [
        "元素伤害",
        "雷"
    ],
    func: expectFunction("thunder"),
    "for": "common",
    badge,
}