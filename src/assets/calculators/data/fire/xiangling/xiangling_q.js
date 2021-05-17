import { tableNormalA, tableReactionA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";

let skillKeys = [
    {
        key: "dmg1",
        chs: "一段挥舞伤害",
        skill: "q",
        element: "fire",
    },
    {
        key: "dmg2",
        chs: "二段挥舞伤害",
        skill: "q",
        element: "fire",
    },
    {
        key: "dmg3",
        chs: "三段挥舞伤害",
        skill: "q",
        element: "fire",
    },
    {
        key: "dmg4",
        chs: "旋火轮伤害",
        skill: "q",
        element: "fire",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let fireBonus = c.constellation >= 6 ? 0.15 : 0;
    attribute.fireBonus += fireBonus;

    let ret = mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["fire", tableNormalA(attribute, configObject, enemy, skillKeys, "q")],
        ["fireMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys, "q")],
        ["fireVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys, "q")],
    );

    return ret;
}