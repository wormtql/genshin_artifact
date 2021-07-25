import { rowsAir, tablePhysical, tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rows1 = [
    { key: "dmg1", chs: "普攻1段/2" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg4", chs: "普攻4段/2" },
    { key: "dmg5", chs: "普攻5段" },
];

let rows2 = [
    { key: "bDmg1", chs: "瞄准射击" },
];

let rows3 = [
    { key: "bDmg2", chs: "满蓄力瞄准射击" },
    { key: "bDmg3", chs: "焰硝矢伤害" },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let a = tablePhysical(attribute, configObject, enemy, rows1, "a");
    let b = tablePhysical(attribute, configObject, enemy, rows2, "b");
    let b2 = tableFire(attribute, configObject, enemy, rows3, "b");
    let air = tablePhysical(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, b2, air
    };
}