import badge from "./badges/ice_slime.png";
import displayFunction from "../factory/display";

export default {
    name: "iceDisplay",
    chs: "上限-冰元素伤害",
    description: [
        "相当于使得E（冰）能够达到的伤害最大值最高",
    ],
    tags: [
        "表演",
        "冰",
    ],
    func: displayFunction("ice"),
    "for": "common",
    badge,
}