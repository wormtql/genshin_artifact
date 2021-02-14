import badge from "./badges/water_slime.png";
import displayFunction from "../factory/display";

export default {
    name: "waterDisplay",
    chs: "上限-水元素伤害",
    description: [
        "相当于使得E（水）能够达到的伤害最大值最高",
    ],
    tags: [
        "表演",
        "水",
    ],
    func: displayFunction("water"),
    "for": "common",
    badge,
}