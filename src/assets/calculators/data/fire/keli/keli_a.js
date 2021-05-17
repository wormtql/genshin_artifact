import { tableNormalA, tableReactionA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import deepCopy from "@util/deepcopy";
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
        key: "bDmg1",
        chs: "重击",
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

let skillKeys1 = deepCopy(skillKeys);
skillKeys1.forEach(item => item.element = "fire");

let key2 = [
    {
        key: "bDmg1",
        chs: "重击（爆裂火花）",
        skill: "a",
        element: "fire",
    }
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let buffs = configObject.buffs;
    let attribute = getAttribute(artifacts, c, w, buffs);

    // let hasTalent2 = c.level > 60 || (c.level === 60 && c.ascend);
    // if (hasTalent2) {
    //     attribute.fireBonus += 0.2;
    // }

    let temp =  mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["normal", tableNormalA(attribute, configObject, enemy, skillKeys1, "a")],
        ["normalMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys1, "a")],
        ["normalVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys1, "a")],
    );

    attribute.bonus += 0.5;
    let temp2 = mergeArray(
        ["chs", key2.map(item => item.chs)],
        ["normal", tableNormalA(attribute, configObject, enemy, key2, "a")],
        ["normalMelt", tableReactionA("melt", attribute, configObject, enemy, key2, "a")],
        ["normalVaporize", tableReactionA("vaporize", attribute, configObject, enemy, key2, "a")],
    );
    
    temp.push(temp2[0]);
    console.log(temp);

    return temp;
}