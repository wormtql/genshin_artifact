import { rowsAir, tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["yanfei"].skill;


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
];

let rowsB = [
    {
        key: "bDmg1",
        chs: "无印伤害",
    },
    {
        key: "bDmg2",
        chs: "1印伤害",
    },
    {
        key: "bDmg3",
        chs: "2印伤害",
    },
    {
        key: "bDmg4",
        chs: "3印伤害",
    },
    {
        key: "bDmg5",
        chs: "4印伤害",
    },
];

export default function (artifacts, configObject, enemy, otherConfig) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    if (otherConfig.burn) {
        attribute.bBonus += skill.q.bBonus[c.skill3 - 1];
    }

    let a = tableFire(attribute, configObject, enemy, rowsA, "a");
    let b = tableFire(attribute, configObject, enemy, rowsB, "b");
    let air = tableFire(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, air,
    }
}