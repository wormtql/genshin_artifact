import {rowThunder} from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["bachongshenzi"].skill;

let rowsE = [
    { key: "dmg1", chs: "杀生樱伤害·壹阶" },
    { key: "dmg2", chs: "杀生樱伤害·贰阶" },
    { key: "dmg3", chs: "杀生樱伤害·叁阶" },
    { key: "dmg4", chs: "杀生樱伤害·肆阶" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    const hasTalent2 = c.level > 60 || (c.level === 60 && c.ascend)

    if (hasTalent2) {
        let em = attribute.elementalMastery
        attribute.eBonus += em * 0.0015
    }

    let e = [];
    for (let row of rowsE) {
        const atk = attribute.attack();
        const base = atk * skill.e[row.key][c.skill2 - 1];
        e.push(rowThunder(attribute, configObject, enemy, row.chs, "e", base));
    }

    return {
        e
    }
}