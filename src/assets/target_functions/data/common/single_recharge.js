import badge from "./badges/dendro_slime_small.png";

function recharge(attribute) {
    return attribute.recharge;
}

export default {
    name: "recharge",
    chs: "单值-元素充能效率",
    description: [
        "使得总元素充能效率最高"
    ],
    tags: [
        "充能",
    ],
    func: recharge,
    "for": "common",
    badge,
}