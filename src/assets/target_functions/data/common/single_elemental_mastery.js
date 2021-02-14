import badge from "./badges/dendro_slime_small.png";

function em(attribute) {
    return attribute.elementalMastery;
}

export default {
    name: "elementalMastery",
    chs: "单值-元素精通",
    description: [
        "使得总元素精通最高",
    ],
    tags: [
        "元素精通",
        "反应",
    ],
    func: em,
    "for": "common",
    badge,
}