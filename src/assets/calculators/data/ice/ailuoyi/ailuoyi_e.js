import { tableIce } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsE = [
    { key: "dmg1", chs: "冰尘弹" },
    { key: "dmg2", chs: "冷冻炸弹" },
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