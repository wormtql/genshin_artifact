import { rowsAir, tablePhysical, tableFire } from "../../../utils";
// import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";


let rows1 = [
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
    {
        key: "dmg5",
        chs: "普攻5段",
    },
];

let rows2 = [
    {
        key: "bDmg1",
        chs: "瞄准射击",
    },
];

let rows3 = [
    {
        key: "bDmg2",
        chs: "满蓄力瞄准射击",
    },
]

// skillKeys.forEach(item => item.element = "physical");

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

// export function normalA(artifacts, configObject, enemy) {
//     let c = configObject.character;
//     let w = configObject.weapon;
//     let buffs = configObject.buffs;
//     let attribute = getAttribute(artifacts, c, w, buffs);

//     let temp =  mergeArray(
//         ["chs", skillKeys.map(item => item.chs)],
//         ["normal", tableNormalA(attribute, configObject, enemy, skillKeys, "a")],
//     )

//     return temp;
// }

// let skillKeys2 = [
//     {
//         key: "bDmg2",
//         chs: "满蓄力瞄准射击",
//         skill: "b",
//         element: "fire",
//     },
// ]

// export function fireA(artifacts, configObject, enemy) {
//     let c = configObject.character;
//     let w = configObject.weapon;
//     let buffs = configObject.buffs;
//     let attribute = getAttribute(artifacts, c, w, buffs);

//     let temp =  mergeArray(
//         ["chs", skillKeys2.map(item => item.chs)],
//         ["fire", tableNormalA(attribute, configObject, enemy, skillKeys2, "a")],
//         ["fireMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys2, "a")],
//         ["fireVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys2, "a")],
//     );

//     return temp;
// }