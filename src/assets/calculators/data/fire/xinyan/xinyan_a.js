import { tableNormalA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";


let skillKeys1 = [
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

let skillKeys2 = [
    {
        key: "bDmg1",
        chs: "重击循环伤害",
        skill: "b",
        element: "physical",
    },
    {
        key: "bDmg2",
        chs: "重击终结伤害",
        skill: "b",
        element: "physical",
    },
]

skillKeys1.forEach(item => item.element = "physical");

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let a = mergeArray(
        ["chs", skillKeys1.map(item => item.chs)],
        ["normal", tableNormalA(attribute, configObject, enemy, skillKeys1, "a")],
    );

    if (c.constellation >= 6) {
        attribute.attackStatic += 0.5 * attribute.defend();
    }

    let b = mergeArray(
        ["chs", skillKeys2.map(item => item.chs)],
        ["normal", tableNormalA(attribute, configObject, enemy, skillKeys2, "a")],
    );

    return {
        a,
        b,
    };
}