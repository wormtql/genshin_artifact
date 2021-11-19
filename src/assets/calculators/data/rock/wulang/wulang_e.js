import { rowRock } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


const skill = charactersData["wulang"].skill;

const rowsE = [
    { key: "dmg1", chs: "技能伤害" },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let e = [];
    for (let row of rowsE) {
        const atk = attribute.attack();
        const base = atk * skill.e[row.key][c.skill2 - 1];
        e.push(rowRock(attribute, configObject, enemy, row.chs, "e", base));
    }

    return {
        e,
    }
}