import { rowIce } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

const skill = charactersData["shenhe"].skill;

const rowsQ = [
    { key: "dmg1", chs: "技能伤害" },
    { key: "dmg2", chs: "持续伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let q = [];
    for (let { key, chs } of rowsQ) {
        const atk = attribute.attack();
        const base = atk * skill.q[key][c.skill3 - 1];
        q.push(rowIce(attribute, configObject, enemy, chs, "q", base));
    }

    return {
        q,
    }
}