import { rowsAir, tableFire, tablePhysical } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsA = [
    {
        key: "dmg1",
        chs: "普攻1段",
    },
    {
        key: "dmg2",
        chs: "普攻2段",
    },
    {
        key: "dmg3",
        chs: "普攻3段",
    },
    {
        key: "dmg4",
        chs: "普攻4段",
    },
];

let rowsB = [
    {
        key: "bDmg1",
        chs: "重击循环",
    },
    {
        key: "bDmg2",
        chs: "重击终结",
    },
];

export default function (artifacts, configObject, enemy, otherConfig) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let hasTalent2 = c.level > 60 || (c.level === 60 && c.ascend);
    if (hasTalent2) {
        attribute.fireBonus += 0.2;
    }

    let f;
    if (otherConfig.afterQ) {
        f = tableFire;
    } else {
        f = tablePhysical;
    }

    let a = f(attribute, configObject, enemy, rowsA, "a");
    let b = f(attribute, configObject, enemy, rowsB, "b");
    let air = f(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, air,
    };
}