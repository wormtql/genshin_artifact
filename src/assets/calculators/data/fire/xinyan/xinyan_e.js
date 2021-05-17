import { tableNormalA, tableReactionA } from "../../../utils";
import mergeArray from "@util/mergeArray";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let xinyanE = charactersData["xinyan"].skill.e;

let skillKeys = [
    {
        key: "dmg1",
        chs: "挥舞伤害",
        skill: "e",
        element: "fire",
    },
    {
        key: "dmg2",
        chs: "持续伤害",
        skill: "e",
        element: "fire",
    }
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let dmg = mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["fire", tableNormalA(attribute, configObject, enemy, skillKeys, "e")],
        ["fireMelt", tableReactionA("melt", attribute, configObject, enemy, skillKeys, "e")],
        ["fireVaporize", tableReactionA("vaporize", attribute, configObject, enemy, skillKeys, "e")],
    );

    let index = c.skill2 - 1;
    let shield1 = attribute.defend() * xinyanE.shield1[index] + xinyanE.shield1Static[index];
    let shield2 = attribute.defend() * xinyanE.shield2[index] + xinyanE.shield2Static[index];
    let shield3 = attribute.defend() * xinyanE.shield3[index] + xinyanE.shield3Static[index];

    return {
        dmg,
        shield1,
        shield2,
        shield3,
    };
}