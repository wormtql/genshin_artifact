import { rowsAir, tablePhysical, tableThunder } from "../../../utils";
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
        key: "dmg41",
        chs: "普攻4段-1",
    },
    {
        key: "dmg42",
        chs: "普攻4段-2",
    },
    {
        key: "dmg5",
        chs: "普攻5段",
    },
];

let rowsB = [
    {
        key: "bDmg1",
        chs: "重击-1",
    },
    {
        key: "bDmg2",
        chs: "重击-2",
    },
]

export default function (artifacts, configObject, enemy, otherConfig) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);
    
    let f = otherConfig.afterE ? tableThunder : tablePhysical;

    let a = f(attribute, configObject, enemy, rowsA, "a");
    let b = f(attribute, configObject, enemy, rowsB, "b");
    let air = f(attribute, configObject, enemy, rowsAir, "air");

    return {
        a, b, air,
    }
}