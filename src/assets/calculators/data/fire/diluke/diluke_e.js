import { tableNormalA, tableReactionA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";

let skillKeys = [
    {
        key: "dmg1",
        chs: "一段伤害",
        skill: "e",
        element: "fire",
    },
    {
        key: "dmg2",
        chs: "二段伤害",
        skill: "e",
        element: "fire",
    },
    {
        key: "dmg3",
        chs: "三段伤害",
        skill: "e",
        element: "fire",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let ret = mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["normal", tableNormalA(attribute, configObject, enemy, skillKeys, "e")],
        ["normalMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys, "e")],
        ["normalVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys, "e")],
    );

    return ret;
}