import badge from "./badges/wind_slime.png";
import displayFunction from "../factory/display";

export default {
    name: "windDisplay",
    chs: "上限-风元素伤害",
    description: [
        "相当于使得E（风）能够达到的伤害最大值最高",
    ],
    tags: [
        "表演",
        "风",
    ],
    func: displayFunction("wind"),
    "for": "common",
    badge,
}