import { tableNormalA, tableReactionA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";

let skillKeys = [
    {
        key: "dmg1",
        chs: "斩击伤害",
        skill: "q",
        element: "fire",
    },
    {
        key: "dmg2",
        chs: "持续伤害",
        skill: "q",
        element: "fire",
    },
    {
        key: "dmg3",
        chs: "爆裂伤害",
        skill: "q",
        element: "fire",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let ret = mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["normal", tableNormalA(attribute, configObject, enemy, skillKeys, "q")],
        ["normalMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys, "q")],
        ["normalVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys, "q")],
    );

    return ret;
}