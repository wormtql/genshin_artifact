import { rowWater } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["shanhugongxinhai"].skill;


let rowsE = [
    { key: "dmg1", chs: "波纹伤害", },
]

export default function (artifacts, configObject, enemy, { afterQ }) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let skill2 = c.skill2;
    let skill3 = c.skill3;
    // let hasTalent2 = c.level > 60 || (c.level === 60 && c.ascend);

    let bonusE = 0;
    if (afterQ) {
        bonusE = skill.q.eBonus[skill3 - 1] * attribute.life();
    }

    let tableE = [];
    for (let config of rowsE) {
        let ratio = skill.e[config.key][skill2 - 1];
        let base = ratio * attribute.attack() + bonusE;
        let row = rowWater(attribute, configObject, enemy, config.chs, "e", base);
        tableE.push(row);
    }

    let heal = attribute.life() * skill.e.heal1[skill2 - 1] + skill.e.heal1Fixed[skill2 - 1];
    heal *= (1 + attribute.cureEffect);

    return {
        e: tableE,
        heal,
    }
}