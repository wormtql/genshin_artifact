import { rowsAir, rowPhysical } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters"


const skill = charactersData["abeiduo"].skill;

let rowsA = [
    { key: "dmg1", chs: "普攻1段" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg4", chs: "普攻4段" },
    { key: "dmg5", chs: "普攻5段" },
];

let rowsB = [
    { key: "bDmg11", chs: "重击伤害-1" },
    { key: "bDmg12", chs: "重击伤害-2" },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let a = [];
    for (let row of rowsA) {
        const atk = attribute.attack();
        const ratio = skill.a[row.key][c.skill1 - 1];
        const dmg = atk * ratio;
        a.push(rowPhysical(attribute, configObject, enemy, row.chs, "a", dmg));
    }

    let b = [];
    for (let row of rowsB) {
        const atk = attribute.attack();
        const ratio = skill.a[row.key][c.skill1 - 1];
        const dmg = atk * ratio;
        b.push(rowPhysical(attribute, configObject, enemy, row.chs, "b", dmg));
    }

    let air = [];
    for (let row of rowsAir) {
        const atk = attribute.attack();
        const ratio = skill.a[row.key][c.skill1 - 1];
        const dmg = atk * ratio;
        air.push(rowPhysical(attribute, configObject, enemy, row.chs, "air", dmg));
    }

    return {
        a, b, air,
    }
}