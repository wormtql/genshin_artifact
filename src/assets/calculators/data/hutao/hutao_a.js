import { tableNormal, tableNormalA, tableReactionA } from "../../utils";
import mergeArray from "@util/mergeArray";
import deepCopy from "@util/deepcopy";
import { getAttribute } from "@util/attribute";


import { charactersData } from "@asset/characters";

const hutao = charactersData["hutao"];

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
        key: "dmg51",
        chs: "普攻5段-1",
        skill: "a",
    },
    {
        key: "dmg52",
        chs: "普攻5段-2",
        skill: "a",
    },
    {
        key: "dmg6",
        chs: "普攻6段",
        skill: "a",
    },
    {
        key: "bDmg",
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
skillKeys1.forEach(item => item.element = "physical");

let skillKeys2 = deepCopy(skillKeys);
skillKeys2.forEach(item => item.element = "fire");

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let buffs = configObject.buffs;
    let attribute = getAttribute(artifacts, c, w, buffs);

    let temp =  mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["normal", tableNormal(artifacts, configObject, enemy, skillKeys1, "a")],
        // ["thunder", tableNormal(artifacts, configObject, enemy, skillKeys2, "a")],
    )

    // 开E后
    attribute.attackStatic += hutao.skill.e.hp[c.skill2 - 1] * attribute.life();
    temp = mergeArray(
        temp,
        ["afterE", tableNormalA(attribute, configObject, enemy, skillKeys2, "a")],
        ["afterEMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys2, "a")],
        ["afterEVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys2, "a")]
    );

    // 开E后融化


    // 开E且生命值低于50
    // attribute.fireBonus += 0.33;
    // temp = mergeArray(
    //     temp,
    //     ["afterEBelow50", tableNormalA(attribute, configObject, enemy, skillKeys2, "a")],
    // );
    // console.log(temp);

    return temp;
}