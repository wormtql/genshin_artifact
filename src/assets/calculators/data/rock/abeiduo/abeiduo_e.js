import { rowRock } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


const skill = charactersData["abeiduo"].skill;

const rowsE1 = [
    { key: "dmg1", chs: "技能伤害" },
];

const rowsE2 = [
    { key: "dmg2", chs: "刹那之花伤害" }
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let e = [];
    for (const row of rowsE1) {
        const atk = attribute.attack();
        const ratio = skill.e[row.key][c.skill2 - 1];
        const base = atk * ratio;
        e.push(rowRock(attribute, configObject, enemy, row.chs, "e", base));
    }

    for (const row of rowsE2) {
        const def = attribute.defend();
        const ratio = skill.e[row.key][c.skill2 - 1];
        const base = def * ratio;
        e.push(rowRock(attribute, configObject, enemy, row.chs, "e", base));
    }

    return {
        e
    };
}