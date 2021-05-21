import { rowsAir, tablePhysical, tableWind } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["xiao"].skill;

let rowsA = [
    { key: "dmg11", chs: "普攻1段-1" },
    { key: "dmg12", chs: "普攻1段-2" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg41", chs: "普攻4段-1" },
    { key: "dmg42", chs: "普攻4段-2" },
    { key: "dmg5", chs: "普攻5段" },
    { key: "dmg6", chs: "普攻6段" },
];

let rowsB = [
    { key: "bDmg1", chs: "重击伤害" },
]

export default function (artifacts, configObject, enemy, otherConfig) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let f;
    if (otherConfig.afterQ) {
        let bonus = skill.q.bonus[c.skill3 - 1];
        attribute.aBonus += bonus;
        attribute.bBonus += bonus;
        attribute.airBonus += bonus;
        f = tableWind;
    } else {
        f = tablePhysical;
    }

    let a = f(attribute, configObject, enemy, rowsA, "a");
    let b = f(attribute, configObject, enemy, rowsB, "b");
    let air = f(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, air,
    }
}