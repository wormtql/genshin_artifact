import badge from "./badges/rock_slime.png";
import expectFunction from "../factory/expect";

export default {
    name: "rockExpect",
    chs: "期望-岩元素伤害",
    description: [
        "相当于使得多次E（岩）的伤害平均值最高",
    ],
    tags: [
        "元素伤害",
        "岩"
    ],
    func: expectFunction("rock"),
    "for": "common",
    badge,
}