import { rowsAir, tablePhysical, tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

const skill = charactersData["hutao"].skill;

let rowsA = [
    {
        key: "dmg1",
        chs: "普攻1段",
    },
    {
        key: "dmg2",
        chs: "普攻2段",
    },
    {
        key: "dmg3",
        chs: "普攻3段",
    },
    {
        key: "dmg4",
        chs: "普攻4段",
    },
    {
        key: "dmg51",
        chs: "普攻5段-1",
    },
    {
        key: "dmg52",
        chs: "普攻5段-2",
    },
    {
        key: "dmg6",
        chs: "普攻6段",
    },
];

let rowsB = [
    {
        key: "bDmg",
        chs: "重击",
        skill: "b",
    },
];

export default function (artifacts, configObject, enemy, otherConfig) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let f = otherConfig.afterE ? tableFire : tablePhysical;
    if (otherConfig.afterE) {
        let atkBonus = Math.min(4 * attribute.attackBasic, skill.e.hp[c.skill2 - 1] * attribute.life());
        attribute.attackStatic += atkBonus;
    }

    let a = f(attribute, configObject, enemy, rowsA, "a");
    let b = f(attribute, configObject, enemy, rowsB, "b");
    let air = f(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, air
    };
}