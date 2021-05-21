import { tableWind } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsE = [
    { key: "dmg1", chs: "初始切割伤害" },
    { key: "dmg2", chs: "最大切割伤害" },
    { key: "dmg3", chs: "初始爆风伤害" },
    { key: "dmg4", chs: "最大爆风伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let e = tableWind(attribute, configObject, enemy, rowsE, "e");

    return e;
}