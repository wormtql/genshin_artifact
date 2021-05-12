/**
 * 刻晴
 * E
 */

import { tableNormal } from "../../utils";
import mergeArray from "@util/mergeArray";


let skillKeys = [
    {
        key: "dmg1",
        chs: "技能伤害",
        skill: "q",
        element: "thunder",
    },
    {
        key: "dmg2",
        chs: "连斩伤害",
        skill: "q",
        element: "thunder",
    },
    {
        key: "dmg3",
        chs: "最后一击",
        skill: "q",
        element: "thunder",
    },
];

export default function (artifacts, configObject, enemy) {
    return mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["normal", tableNormal(artifacts, configObject, enemy, skillKeys, "q")],
    )
}