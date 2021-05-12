/**
 * 刻晴
 * E
 */

import { tableNormal } from "../../utils";
import mergeArray from "@util/mergeArray";

let skillKeys = [
    {
        key: "dmg1",
        chs: "雷楔伤害",
        skill: "e",
        element: "thunder",
    },
    {
        key: "dmg2",
        chs: "斩击伤害",
        skill: "e",
        element: "thunder",
    }
];

export default function (artifacts, configObject, enemy) {
    return mergeArray(
        ["chs", skillKeys.map(item => item.chs)],
        ["normal", tableNormal(artifacts, configObject, enemy, skillKeys, "e")],
    )
}