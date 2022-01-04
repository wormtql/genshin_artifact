import { rowsAir, rowPhysical } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

const skill = charactersData["shenhe"].skill;

const rowsA = [
    { key: "dmg1", chs: "普攻1段" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg41", chs: "普攻4段-1" },
    { key: "dmg42", chs: "普攻4段-2" },
    { key: "dmg5", chs: "普攻5段" },
];

const rowsB = [
    { key: "bDmg1", chs: "重击伤害" },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let a = [];
    for (let { key, chs } of rowsA) {
        const atk = attribute.attack();
        const base = atk * skill.a[key][c.skill1 - 1];
        a.push(rowPhysical(attribute, configObject, enemy, chs, "a", base));
    }

    let b = [];
    for (let { key, chs } of rowsB) {
        const atk = attribute.attack();
        const base = atk * skill.a[key][c.skill1 - 1];
        b.push(rowPhysical(attribute, configObject, enemy, chs, "b", base));
    }

    let air = [];
    for (let { key, chs } of rowsAir) {
        const atk = attribute.attack();
        const base = atk * skill.a[key][c.skill1 - 1];
        air.push(rowPhysical(attribute, configObject, enemy, chs, "air", base));
    }

    return {
        a, b, air,
    }
}