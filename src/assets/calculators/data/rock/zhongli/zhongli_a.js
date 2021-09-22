import { rowsAir, rowPhysical } from "../../../utils";
import { getAttribute } from "@util/attribute";
// import { tablePhysical } from "./zhongli_utils";
import { charactersData } from "@asset/characters";


let skill = charactersData["zhongli"].skill;

let rowsA = [
    { key: "dmg1", chs: "普攻1段" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg4", chs: "普攻4段" },
    { key: "dmg5", chs: "普攻5段/4" },
    { key: "dmg6", chs: "普攻6段" },
];

let rowsB = [
    {
        key: "bDmg1",
        chs: "重击伤害",
    },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let skill1 = c.skill1;

    let hasTalent2 = (c.level == 60 && c.ascend) || c.level > 60;
    let extraDmg = 0;
    if (hasTalent2) {
        extraDmg = attribute.life() * 0.0139;
    }

    let tableA = [];
    for (let config of rowsA) {
        let ratio = skill.a[config.key][skill1 - 1];
        let base = ratio * attribute.attack() + extraDmg;
        let row = rowPhysical(attribute, configObject, enemy, config.chs, "a", base);
        tableA.push(row);
    }

    let tableB = [];
    for (let config of rowsB) {
        let ratio = skill.a[config.key][skill1 - 1];
        let base = ratio * attribute.attack() + extraDmg;
        let row = rowPhysical(attribute, configObject, enemy, config.chs, "b", base);
        tableB.push(row);
    }

    let tableAir = [];
    for (let config of rowsAir) {
        let ratio = skill.a[config.key][skill1 - 1];
        let base = ratio * attribute.attack() + extraDmg;
        let row = rowPhysical(attribute, configObject, enemy, config.chs, "air", base);
        tableAir.push(row);
    }

    return {
        a: tableA, b: tableB, air: tableAir,
    }
}