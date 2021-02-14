import badge from "./badges/dendro_slime_small.png";

function life(attribute) {
    return attribute.life();
}

export default {
    name: "life",
    chs: "单值-生命值",
    description: [
        "使得总生命值最高",
    ],
    tags: [
        "生命值",
        "生存",
    ],
    func: life,
    "for": "common",
    badge,
}