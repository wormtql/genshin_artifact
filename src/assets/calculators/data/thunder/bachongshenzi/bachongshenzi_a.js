import {rowsAir, rowThunder} from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

const skill = charactersData["bachongshenzi"].skill

let rowsA = [
    { key: "dmg1", chs: "普攻1段" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
];

let rowsB = [
    { key: "bDmg1", chs: "重击伤害" },
]

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let a = []
    for (let item of rowsA) {
        let atk = attribute.attack()
        let base = atk * skill.a[item.key][c.skill1 - 1]
        a.push(rowThunder(attribute, configObject, enemy, item.chs, "a", base))
    }

    let b = []
    for (let item of rowsB) {
        let atk = attribute.attack()
        let base = atk * skill.a[item.key][c.skill1 - 1]
        b.push(rowThunder(attribute, configObject, enemy, item.chs, "b", base))
    }

    let air = []
    for (let item of rowsAir) {
        let atk = attribute.attack()
        let base = atk * skill.a[item.key][c.skill1 - 1]
        air.push(rowThunder(attribute, configObject, enemy, item.chs, "air", base))
    }

    return {
        a, b, air,
    }
}
