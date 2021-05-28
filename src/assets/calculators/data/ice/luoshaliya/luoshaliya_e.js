import { tableIce } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsE = [
    { key: "dmg11", chs: "技能伤害-1" },
    { key: "dmg12", chs: "技能伤害-2" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let e = tableIce(attribute, configObject, enemy, rowsE, "e");
    
    return {
        e
    }
}