import badge from "./badges/water_slime.png";
import expectFunction from "../factory/expect";

export default {
    name: "waterExpect",
    chs: "期望-水元素伤害",
    description: [
        "相当于使得多次E（水）的伤害平均值最高",
    ],
    tags: [
        "元素伤害",
        "水"
    ],
    func: expectFunction("water"),
    "for": "common",
    badge,
}