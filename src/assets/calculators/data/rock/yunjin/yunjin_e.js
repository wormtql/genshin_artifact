import { rowRock } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


const skill = charactersData["yunjin"].skill;

const rowsE = [
    { key: "dmg1", chs: "点按伤害" },
    { key: "dmg2", chs: "一段蓄力伤害" },
    { key: "dmg3", chs: "二段蓄力伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let e = [];
    for (let { key, chs } of rowsE) {
        const def = attribute.defend();
        const base = def * skill.e[key][c.skill2 - 1];
        e.push(rowRock(attribute, configObject, enemy, chs, "e", base));
    }

    const hp = attribute.life();
    const shield = hp * skill.e.shield1[c.skill2 - 1] + skill.e.shield1Fixed[c.skill2 - 1];

    return {
        e, shield,
    }
}