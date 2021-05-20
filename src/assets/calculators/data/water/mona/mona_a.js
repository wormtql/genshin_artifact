import { rowsAir, tableWater } from "../../../utils";
import { getAttribute } from "@util/attribute";

let rowsA = [
    {
        key: "dmg1",
        chs: "普攻1段",
    },
    {
        key: "dmg2",
        chs: "普攻2段",
    },
    {
        key: "dmg3",
        chs: "普攻3段",
    },
    {
        key: "dmg4",
        chs: "普攻4段",
    },
];

let rowsB = [
    {
        key: "bDmg1",
        chs: "重击伤害",
    },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let a = tableWater(attribute, configObject, enemy, rowsA, "a");
    let b = tableWater(attribute, configObject, enemy, rowsB, "b");
    let air = tableWater(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, air,
    }
}