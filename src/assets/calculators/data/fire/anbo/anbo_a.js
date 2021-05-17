import { tableNormalA, tableReactionA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";


let skillKeys = [
    {
        key: "dmg1",
        chs: "普攻1段",
        skill: "a",
    },
    {
        key: "dmg2",
        chs: "普攻2段",
        skill: "a",
    },
    {
        key: "dmg3",
        chs: "普攻3段",
        skill: "a",
    },
    {
        key: "dmg4",
        chs: "普攻4段",
        skill: "a",
    },
    {
        key: "dmg5",
        chs: "普攻5段",
        skill: "a",
    },
    {
        key: "bDmg1",
        chs: "瞄准射击",
        skill: "b",
    },
    {
        key: "airDmg1",
        chs: "下坠期间",
        skill: "air",
    },
    {
        key: "airDmg2",
        chs: "下坠（低空）",
        skill: "air",
    },
    {
        key: "airDmg3",
        chs: "下坠（高空）",
        skill: "air",
    }
];

skillKeys.forEach(item => item.element = "physical");

export function normalA(artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let buffs = configObject.buffs;
    let attribute = getAttribute(artifacts, c, w, buffs);

    let temp =  mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["normal", tableNormalA(attribute, configObject, enemy, skillKeys, "a")],
    )

    return temp;
}

let skillKeys2 = [
    {
        key: "bDmg2",
        chs: "满蓄力瞄准射击",
        skill: "b",
        element: "fire",
    },
]

export function fireA(artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let buffs = configObject.buffs;
    let attribute = getAttribute(artifacts, c, w, buffs);

    let temp =  mergeArray(
        ["chs", skillKeys2.map(item => item.chs)],
        ["fire", tableNormalA(attribute, configObject, enemy, skillKeys2, "a")],
        ["fireMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys2, "a")],
        ["fireVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys2, "a")],
    );

    return temp;
}