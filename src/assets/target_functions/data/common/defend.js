import badge from "./badges/badge_defend.png";

function defend(attribute) {
    return attribute.defend();
}

export default {
    name: "defend",
    chs: "防御力",
    description: [
        "使得总防御力最高",
    ],
    tags: [
        "防御",
        "通用",
    ],
    func: defend,
    "for": "common",
    badge,
}