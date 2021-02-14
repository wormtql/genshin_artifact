import badge from "./badges/fire_slime.png";
import expectFunction from "../factory/expect";

export default {
    name: "fireExpect",
    chs: "期望-火元素伤害",
    description: [
        "相当于使得多次E（火）的伤害平均值最高",
    ],
    tags: [
        "元素伤害",
        "火",
    ],
    func: expectFunction("fire"),
    "for": "common",
    badge,
}