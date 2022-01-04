import { rowRock } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


const skill = charactersData["yunjin"].skill;

const rowsQ = [
    { key: "dmg1", chs: "技能伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let q = [];
    for (let { key, chs } of rowsQ) {
        const atk = attribute.attack();
        const base = atk * skill.q[key][c.skill3 - 1];
        q.push(rowRock(attribute, configObject, enemy, chs, "q", base));
    }

    return {
        q,
    }
}