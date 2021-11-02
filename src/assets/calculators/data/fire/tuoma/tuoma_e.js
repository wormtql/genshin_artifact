import { tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


const skill = charactersData["tuoma"].skill;

const skillKeys = [
    { key: "dmg1", chs: "技能伤害" },
];

export default function (artifacts, configObject, enemy) {
    const c = configObject.character;
    const w = configObject.weapon;
    const attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    const e = tableFire(attribute, configObject, enemy, skillKeys, "e");

    const skill2 = c.skill2;
    const shield1 = attribute.life() * skill.e.shield1[skill2 - 1] + skill.e.shield1Fixed[skill2 - 1];
    const shield2 = attribute.life() * skill.e.shield2[skill2 - 1] + skill.e.shield2Fixed[skill2 - 1];

    return {
        e, shield1, shield2
    }
}