/**
 * 刻晴
 * 普攻、重击，不附魔
 */

import { tableNormal } from "../../utils";
import mergeArray from "@util/mergeArray";
import deepCopy from "@util/deepcopy";

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
        key: "dmg41",
        chs: "普攻4段-1",
        skill: "a",
    },
    {
        key: "dmg42",
        chs: "普攻4段-2",
        skill: "a",
    },
    {
        key: "dmg5",
        chs: "普攻5段",
        skill: "a",
    },
    {
        key: "bDmg1",
        chs: "重击-1",
        skill: "b",
    },
    {
        key: "bDmg2",
        chs: "重击-2",
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
skillKeys2.forEach(item => item.element = "thunder");

export default function (artifacts, configObject, enemy) {
    
    
    return mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["normal", tableNormal(artifacts, configObject, enemy, skillKeys1, "a")],
        ["thunder", tableNormal(artifacts, configObject, enemy, skillKeys2, "a")],
    )
    
    // return ret;
}