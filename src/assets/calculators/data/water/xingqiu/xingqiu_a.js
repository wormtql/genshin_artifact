import { rowsAir, tablePhysical } from "../../../utils";
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
        key: "dmg31",
        chs: "普攻3段-1",
    },
    {
        key: "dmg32",
        chs: "普攻3段-2",
    },
    {
        key: "dmg4",
        chs: "普攻4段",
    },
    {
        key: "dmg51",
        chs: "普攻5段-1",
    },
    {
        key: "dmg52",
        chs: "普攻5段-2",
    },
];

let rowsB = [
    {
        key: "bDmg11",
        chs: "重击伤害-1",
    },
    {
        key: "bDmg12",
        chs: "重击伤害-2",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let a = tablePhysical(attribute, configObject, enemy, rowsA, "a");
    let b = tablePhysical(attribute, configObject, enemy, rowsB, "b");
    let air = tablePhysical(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, air,
    }
}