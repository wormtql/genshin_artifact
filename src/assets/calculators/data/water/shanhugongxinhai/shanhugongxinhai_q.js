import { rowWater } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["shanhugongxinhai"].skill;


let rowsQ = [
    { key: "dmg1", chs: "技能伤害", },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let skill3 = c.skill3;

    let tableQ = [];
    for (let config of rowsQ) {
        let ratio = skill.q[config.key][skill3 - 1];
        let base = ratio * attribute.life();
        let row = rowWater(attribute, configObject, enemy, config.chs, "q", base);
        row.tag = "expectOnly";
        tableQ.push(row);
    }

    let heal = attribute.life() * skill.q.heal1[skill3 - 1] + skill.q.heal1Fixed[skill3 - 1];
    heal *= 1 + attribute.cureEffect;

    return {
        q: tableQ,
        heal,
    }
}