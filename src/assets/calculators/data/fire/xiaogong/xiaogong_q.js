import { tableFire } from "../../../utils";
import { getAttribute } from "@util/attribute";


let skillKeys = [
    { key: "dmg1", chs: "技能伤害" },
    { key: "dmg2", chs: "琉金火光爆炸伤害" },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let q = tableFire(attribute, configObject, enemy, skillKeys, "q");

    return {
        q
    }
}