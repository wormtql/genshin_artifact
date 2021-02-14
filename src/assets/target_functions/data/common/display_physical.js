import badge from "./badges/sword.png";
import displayFunction from "../factory/display";

export default {
    name: "physicalDisplay",
    chs: "上限-物理伤害",
    description: [
        "相当于使得平A能够达到的伤害最大值最高",
    ],
    tags: [
        "表演",
        "物理",
    ],
    func: displayFunction("physical"),
    "for": "common",
    badge,
}