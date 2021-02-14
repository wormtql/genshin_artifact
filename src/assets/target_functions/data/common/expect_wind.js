import badge from "./badges/wind_slime.png";
import expectFunction from "../factory/expect";

export default {
    name: "windExpect",
    chs: "期望-风元素伤害",
    description: [
        "相当于使得多次E（风）的伤害平均值最高",
    ],
    tags: [
        "元素伤害",
        "风"
    ],
    func: expectFunction("wind"),
    "for": "common",
    badge,
}