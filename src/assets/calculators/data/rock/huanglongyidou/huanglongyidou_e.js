import { rowRock } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["huanglongyidou"].skill;

let rowsE = [
    { key: "dmg1", chs: "技能伤害" },
];

export default function (artifacts, configObject, enemy, { afterQ }) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    if (afterQ) {
        attribute.attackPercentage += attribute.defend() * skill.q.atkBonus[c.skill3 - 1];
    }

    let e = [];
    for (let row of rowsE) {
        const atk = attribute.attack();
        const base = atk * skill.e[row.key][c.skill2 - 1];
        e.push(rowRock(attribute, configObject, enemy, row.chs, "e", base));
    }

    return {
        e
    }
}