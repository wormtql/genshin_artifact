import badge from "./badges/badge_defend.png";

function life(attribute) {
    return attribute.life();
}

export default {
    name: "life",
    chs: "生命值",
    description: [
        "使得总生命值最高",
    ],
    tags: [
        "生命值",
        "生存",
        "通用",
    ],
    func: life,
    "for": "common",
    badge,
}