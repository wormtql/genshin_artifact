import badge from "./badges/ice_slime.png";
import expectFunction from "../factory/expect";

export default {
    name: "iceExpect",
    chs: "期望-冰元素伤害",
    description: [
        "相当于使得多次E（冰）的伤害平均值最高",
    ],
    tags: [
        "元素伤害",
        "冰"
    ],
    func: expectFunction("ice"),
    "for": "common",
    badge,
}