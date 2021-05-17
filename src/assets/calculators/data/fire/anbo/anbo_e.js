import { tableNormalA, tableReactionA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";

let skillKeys = [
    {
        key: "dmg1",
        chs: "爆炸伤害",
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
        ["fire", tableNormalA(attribute, configObject, enemy, skillKeys, "e")],
        ["fireMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys, "e")],
        ["fireVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys, "e")],
    );

    return ret;
}