import { tableNormalA, tableReactionA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";

let skillKeys1 = [
    {
        key: "dmg2",
        chs: "火元素持续伤害",
        skill: "q",
        element: "physical",
    },
];

let skillKeys2 = [
    {
        key: "dmg1",
        chs: "技能伤害",
        skill: "q",
        element: "physical",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let fire = mergeArray(
        ["chs", skillKeys1.map(item => item.chs)],
        ["fire", tableNormalA(attribute, configObject, enemy, skillKeys1, "q")],
        ["fireMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys1, "q")],
        ["fireVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys1, "q")],
    );

    if (c.constellation >= 2) {
        attribute.qCritical = 1;
    }

    let physical = mergeArray(
        ["chs", skillKeys2.map(item => item.chs)],
        ["normal", tableNormalA(attribute, configObject, enemy, skillKeys2, "q")],
    );

    return {
        fire,
        physical,
    };
}