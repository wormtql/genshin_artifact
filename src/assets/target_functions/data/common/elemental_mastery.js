import badge from "./badges/badge_defend.png";

function em(attribute) {
    return attribute.elementalMastery;
}

export default {
    name: "elementalMastery",
    chs: "元素精通",
    description: [
        "使得总元素精通最高",
    ],
    tags: [
        "精通",
        "反应",
    ],
    func: em,
    "for": "common",
    badge,
}