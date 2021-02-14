import badge from "./badges/fire_slime.png";
import displayFunction from "../factory/display";

export default {
    name: "fireDisplay",
    chs: "上限-火元素伤害",
    description: [
        "相当于使得E（火）能够达到的伤害最大值最高",
    ],
    tags: [
        "表演",
        "火",
    ],
    func: displayFunction("fire"),
    "for": "common",
    badge,
}