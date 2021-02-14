import badge from "./badges/rock_slime.png";
import displayFunction from "../factory/display";

export default {
    name: "rockDisplay",
    chs: "上限-岩元素伤害",
    description: [
        "相当于使得E（岩）能够达到的伤害最大值最高",
    ],
    tags: [
        "表演",
        "岩",
    ],
    func: displayFunction("rock"),
    "for": "common",
    badge,
}