import { rowsAir, tableRock } from "../../../utils";
import { getAttribute } from "@util/attribute";

let rowsA = [
    { key: "dmg1", chs: "普攻伤害" },
    { key: "dmg2", chs: "星璇伤害" },
];

let rowsB = [
    { key: "bDmg1", chs: "重击伤害" },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let a = tableRock(attribute, configObject, enemy, rowsA, "a");
    let b = tableRock(attribute, configObject, enemy, rowsB, "b");
    let air = tableRock(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, air,
    }
}