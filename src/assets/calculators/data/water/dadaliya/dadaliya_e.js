import { tableWater } from "../../../utils";
import { getAttribute } from "@util/attribute";

let rowsA = [
    {
        key: "dmg1",
        chs: "1段伤害",
    },
    {
        key: "dmg2",
        chs: "2段伤害",
    },
    {
        key: "dmg3",
        chs: "3段伤害",
    },
    {
        key: "dmg4",
        chs: "4段伤害",
    },
    {
        key: "dmg5",
        chs: "5段伤害",
    },
    {
        key: "dmg61",
        chs: "6段伤害-1",
    },
    {
        key: "dmg62",
        chs: "6段伤害-2",
    },
];

let rowsB = [
    {
        key: "bDmg11",
        chs: "重击伤害-1"
    },
    {
        key: "bDmg12",
        chs: "重击伤害-2"
    }
];

let rowsE = [
    {
        key: "eDmg1",
        chs: "状态爆发伤害"
    },
    {
        key: "eDmg2",
        chs: "断流-斩"
    },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let a = tableWater(attribute, configObject, enemy, rowsA, "e", "a");
    let b = tableWater(attribute, configObject, enemy, rowsB, "e", "b");
    let e = tableWater(attribute, configObject, enemy, rowsE, "e");

    return {
        a, b, e,
    }
}