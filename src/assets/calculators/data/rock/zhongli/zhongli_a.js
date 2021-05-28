import { rowsAir } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { tablePhysical } from "./zhongli_utils";

let rowsA = [
    { key: "dmg1", chs: "普攻1段" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg4", chs: "普攻4段" },
    { key: "dmg5", chs: "普攻5段/4" },
    { key: "dmg6", chs: "普攻6段" },
];

let rowsB = [
    {
        key: "bDmg1",
        chs: "重击伤害",
    },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let hasTalent2 = (c.level == 60 && c.ascend) || c.level > 60;
    let extraDmg = 0;
    if (hasTalent2) {
        extraDmg = attribute.life() * 0.0139;
    }

    let a = tablePhysical(attribute, configObject, enemy, rowsA, "a", extraDmg);
    let b = tablePhysical(attribute, configObject, enemy, rowsB, "b", extraDmg);
    let air = tablePhysical(attribute, configObject, enemy, rowsAir, "air", extraDmg);

    return {
        a, b, air,
    }
}