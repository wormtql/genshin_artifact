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
        key: "dmg4",
        chs: "普攻4段",
        skill: "a",
    },
    {
        key: "bDmg1",
        chs: "重击循环",
        skill: "b",
    },
    {
        key: "bDmg2",
        chs: "重击终结",
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

    let hasTalent2 = c.level > 60 || (c.level === 60 && c.ascend);
    if (hasTalent2) {
        attribute.fireBonus += 0.2;
    }

    let temp =  mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["normal", tableNormalA(attribute, configObject, enemy, skillKeys1, "a")],
        ["fire", tableNormalA(attribute, configObject, enemy, skillKeys2, "a")],
        ["fireMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys2, "a")],
        ["fireVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys2, "a")],
    )

    return temp;
}