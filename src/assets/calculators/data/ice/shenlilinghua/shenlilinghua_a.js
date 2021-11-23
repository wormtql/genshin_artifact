import { rowsAir, tableIce, tablePhysical } from "../../../utils";
import { getAttribute } from "@util/attribute";

let rowsA = [
    { key: "dmg1", chs: "普攻1段" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg4", chs: "普攻4段/3" },
    { key: "dmg5", chs: "普攻5段" },
];

let rowsB = [
    { key: "bDmg1", chs: "重击伤害/3" },
]

export default function (artifacts, configObject, enemy, { afterDash, afterTalent1, afterTalent2 }) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    if (afterTalent1) {
        attribute.aBonus += 0.3;
        attribute.bBonus += 0.3;
    }
    if (afterTalent2) {
        attribute.iceBonus += 0.18;
    }

    let a, b, air;
    if (afterDash) {
        a = tableIce(attribute, configObject, enemy, rowsA, "a");
        b = tableIce(attribute, configObject, enemy, rowsB, "b");
        air = tableIce(attribute, configObject, enemy, rowsAir, "air");
    } else {
        a = tablePhysical(attribute, configObject, enemy, rowsA, "a");
        b = tablePhysical(attribute, configObject, enemy, rowsB, "b");
        air = tablePhysical(attribute, configObject, enemy, rowsAir, "air");
    }

    return {
        a, b, air,
    }
}