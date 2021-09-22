import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";
import { rowRock } from "../../../utils";


let skill = charactersData["zhongli"].skill;

let rowsE = [
    { key: "dmg1", chs: "岩脊伤害" },
    { key: "dmg2", chs: "共鸣伤害" },
    { key: "dmg2", chs: "长按伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let hasTalent2 = (c.level == 60 && c.ascend) || c.level > 60;
    let extraDmg = 0;
    if (hasTalent2) {
        extraDmg = attribute.life() * 0.019;
    }

    let skill2 = c.skill2;

    let tableE = [];
    for (let config of rowsE) {
        let ratio = skill.e[config.key][skill2 - 1];
        let base = ratio * attribute.attack() + extraDmg;
        let row = rowRock(attribute, configObject, enemy, config.chs, "e", base);
        tableE.push(row);
    }

    let shield = skill.e.shieldBase[skill2 - 1] + skill.e.shieldExtra[skill2 - 1] * attribute.life();

    return {
        e: tableE, shield
    }
}