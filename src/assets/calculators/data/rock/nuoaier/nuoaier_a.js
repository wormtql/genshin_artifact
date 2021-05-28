import { rowsAir, tablePhysical, tableRock } from "../../../utils";
import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["nuoaier"].skill;

let rowsA = [
    { key: "dmg1", chs: "普攻1段" },
    { key: "dmg2", chs: "普攻2段" },
    { key: "dmg3", chs: "普攻3段" },
    { key: "dmg4", chs: "普攻4段" },
];

let rowsB = [
    { key: "bDmg1", chs: "重击循环伤害" },
    { key: "bDmg2", chs: "重击终结伤害" },
]

export default function (artifacts, configObject, enemy, otherConfig) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let f = tablePhysical;
    if (otherConfig.afterQ) {
        let atkLift = skill.q.atkLift[c.skill3 - 1] * attribute.defend();
        if (c.constellation >= 6) {
            atkLift += 0.5 * attribute.defend();
        }
        attribute.attackStatic += atkLift;
        f = tableRock;
    }
    let a = f(attribute, configObject, enemy, rowsA, "a");
    let air = f(attribute, configObject, enemy, rowsAir, "air");

    if (c.constellation >= 2) {
        attribute.bBonus += 0.15;
    }
    let b = f(attribute, configObject, enemy, rowsB, "b");
    

    return {
        a, b, air,
    }
}