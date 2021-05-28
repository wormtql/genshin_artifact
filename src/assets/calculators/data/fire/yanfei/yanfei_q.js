import { tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["yanfei"].skill;

let skillKeys = [
    {
        key: "dmg1",
        chs: "技能伤害",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let bBonus = skill.q.bBonus[c.skill3 - 1];
    let q = tableFire(attribute, configObject, enemy, skillKeys, "e");

    return {
        q, bBonus,
    }
}