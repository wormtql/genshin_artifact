import { tableFire, tableIce, tableThunder, tableWater, tableWind } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsE = [
    { key: "dmg1", chs: "风风轮伤害" },
    { key: "dmg2", chs: "风风轮舞踢点按伤害" },
    { key: "dmg3", chs: "风风轮舞踢长按伤害" },
];

let rowsEle = [
    { key: "dmg4", chs: "风风轮附带元素伤害" },
    { key: "dmg4", chs: "风风轮舞踢长按附带元素伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let e = tableWind(attribute, configObject, enemy, rowsE, "e");
    
    let water = tableWater(attribute, configObject, enemy, rowsEle, "e");
    let fire = tableFire(attribute, configObject, enemy, rowsEle, "e");
    let ice = tableIce(attribute, configObject, enemy, rowsEle, "e");
    let thunder = tableThunder(attribute, configObject, enemy, rowsEle, "e");

    return {
        e, water, fire, ice, thunder
    };
}