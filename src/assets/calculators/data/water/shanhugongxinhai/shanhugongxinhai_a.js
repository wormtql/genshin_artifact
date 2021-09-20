import { rowsAir, rowWater } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let skill = charactersData["shanhugongxinhai"].skill;


let rowsA = [
    { key: "dmg1", chs: "普攻1段", },
    { key: "dmg2", chs: "普攻2段", },
    { key: "dmg3", chs: "普攻3段", },
];

let rowsB = [
    { key: "bDmg1", chs: "重击伤害", },
]

export default function (artifacts, configObject, enemy, { afterQ }) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let skill1 = c.skill1;
    let skill3 = c.skill3;
    let hasTalent2 = c.level > 60 || (c.level === 60 && c.ascend);

    let bonusA = 0;
    let bonusB = 0;
    if (afterQ) {
        bonusA = skill.q.aBonus[skill3 - 1] * attribute.life();
        bonusB = skill.q.bBonus[skill3 - 1] * attribute.life();

        if (hasTalent2) {
            bonusA += attribute.cureEffect * 0.15;
            bonusB += attribute.cureEffect * 0.15;
        }
    }

    let tableA = [];
    for (let config of rowsA) {
        let ratio = skill.a[config.key][skill1 - 1];
        let base = ratio * attribute.attack() + bonusA;
        let row = rowWater(attribute, configObject, enemy, config.chs, "a", base);
        tableA.push(row);
    }

    let tableB = [];
    for (let config of rowsB) {
        let ratio = skill.a[config.key][skill1 - 1];
        let base = ratio * attribute.attack() + bonusB;
        let row = rowWater(attribute, configObject, enemy, config.chs, "b", base);
        tableB.push(row);
    }

    let tableAir = [];
    for (let config of rowsAir) {
        let ratio = skill.a[config.key][skill1 - 1];
        let base = ratio * attribute.attack();
        let row = rowWater(attribute, configObject, enemy, config.chs, "air", base);
        tableAir.push(row);
    }

    return {
        a: tableA, b: tableB, air: tableAir,
    }
}