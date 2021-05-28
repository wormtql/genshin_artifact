import { rowsAir, tableFire } from "../../../utils";
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
];

let rowsB = [
    {
        key: "bDmg1",
        chs: "重击",
    },
];

export default function (artifacts, configObject, enemy, otherConfig) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    if (otherConfig.spark) {
        attribute.bBonus += 0.5;
    }

    let a = tableFire(attribute, configObject, enemy, rowsA, "a");
    let b = tableFire(attribute, configObject, enemy, rowsB, "b");
    let air = tableFire(attribute, configObject, enemy, rowsAir, "air");
    
    return {
        a, b, air
    };
}