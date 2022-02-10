import { rowIce } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

const skill = charactersData["shenhe"].skill;

const rowsE = [
    { key: "dmg1", chs: "点按伤害" },
    { key: "dmg2", chs: "长按伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let e = [];
    for (let { key, chs } of rowsE) {
        let atk = attribute.attack();
        let base = atk * skill.e[key][c.skill2 - 1];
        e.push(rowIce(attribute, configObject, enemy, chs, "e", base));
    }

    let bonus = skill.e.bonus[c.skill2 - 1] * attribute.attack();

    return {
        e, bonus,
    }
}