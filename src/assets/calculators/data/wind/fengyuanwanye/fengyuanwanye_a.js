import { rowsAir, tablePhysical, tableWind } from "../../../utils";
import { getAttribute } from "@util/attribute";

let rowsA = [
    { key: "dmg1", chs: "普攻1段" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg31", chs: "普攻3段-1" },
    { key: "dmg32", chs: "普攻3段-2" },
    { key: "dmg4", chs: "普攻4段" },
    { key: "dmg5", chs: "普攻5段/3" },
];

let rowsB = [
    { key: "bDmg1", chs: "重击伤害-1" },
    { key: "bDmg2", chs: "重击伤害-2" },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let a = tablePhysical(attribute, configObject, enemy, rowsA, "a");
    let b = tablePhysical(attribute, configObject, enemy, rowsB, "b");
    let air = tablePhysical(attribute, configObject, enemy, rowsAir, "air");

    let air2 = tableWind(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, air, air2,
    }
}