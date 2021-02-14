import badge from "./badges/thunder_slime.png";
import displayFunction from "../factory/display";

export default {
    name: "thunderDisplay",
    chs: "上限-雷元素伤害",
    description: [
        "相当于使得E（雷）能够达到的伤害最大值最高",
    ],
    tags: [
        "表演",
        "雷",
    ],
    func: displayFunction("thunder"),
    "for": "common",
    badge,
}