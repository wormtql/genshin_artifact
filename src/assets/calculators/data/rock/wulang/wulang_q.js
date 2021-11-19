import { rowRock } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["wulang"].skill;

let rowsQ = [
    { key: "dmg1", chs: "技能伤害" },
    { key: "dmg2", chs: "岩晶崩破伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    const hasTalent2 = c.level > 60 || (c.level === 60 && c.ascend);

    if (hasTalent2) {
        attribute.eDefRatio += 1.56;
        attribute.qDefRatio += 0.156;
    }

    let q = [];
    for (let row of rowsQ) {
        const def = attribute.defend();
        const base = def * skill.q[row.key][c.skill3 - 1];
        q.push(rowRock(attribute, configObject, enemy, row.chs, "q", base));
    }

    return {
        q
    };
}