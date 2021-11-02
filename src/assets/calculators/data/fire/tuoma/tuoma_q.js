import { rowFire } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


const skill = charactersData["tuoma"].skill;

const rowsQ = [
    { key: "dmg1", chs: "技能伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    const hasTalent2 = c.level > 60 || (c.level === 60 && c.ascend);
    const skill3 = c.skill3;

    let q = [];
    for (let item of rowsQ) {
        let baseDmg = attribute.attack() * skill.q[item.key][skill3 - 1];
        let row = rowFire(attribute, configObject, enemy, item.chs, "q", baseDmg);
        q.push(row);
    }

    {
        let baseDmg = attribute.attack() * skill.q["dmg2"][skill3 - 1];
        if (hasTalent2) {
            baseDmg += attribute.life() * 0.022;
        }
        let row = rowFire(attribute, configObject, enemy, "炽火崩破伤害", "q", baseDmg);
        q.push(row);
    }

    let shield1 = attribute.life() * skill.q.shield1[skill3 - 1] + skill.q.shield1Fixed[skill3 - 1];

    return {
        q, shield1,
    }
}