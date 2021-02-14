import badge from "./badges/dendro_slime_small.png";

function defend(attribute) {
    return attribute.defend();
}

export default {
    name: "defend",
    chs: "单值-防御力",
    description: [
        "使得总防御力最高",
    ],
    tags: [
        "防御",
    ],
    func: defend,
    "for": "common",
    badge,
}