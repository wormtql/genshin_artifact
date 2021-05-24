import { rowsAir, tableIce, tablePhysical } from "../../../utils";
import { getAttribute } from "@util/attribute";

let rowsA = [
    { key: "dmg1", chs: "普攻1段" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg4", chs: "普攻4段" },
    { key: "dmg5", chs: "普攻5段" },
];

let rowsB1 = [
    { key: "bDmg1", chs: "瞄准射击" },
];

let rowsB2 = [
    { key: "bDmg2", chs: "满蓄力瞄准射击" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let a = tablePhysical(attribute, configObject, enemy, rowsA, "a");
    let b1 = tablePhysical(attribute, configObject, enemy, rowsB1, "b");
    let b2 = tableIce(attribute, configObject, enemy, rowsB2, "b");
    let air = tablePhysical(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b1, b2, air,
    }
}