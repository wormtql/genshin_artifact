import { tableThunder } from "../../../utils";
import { getAttribute } from "@util/attribute";

let rowsE = [
    {
        key: "dmg1",
        chs: "点按伤害",
    },
    {
        key: "dmg2",
        chs: "长按无引雷伤害",
    },
    {
        key: "dmg3",
        chs: "1层引雷伤害",
    },
    {
        key: "dmg4",
        chs: "2层引雷伤害",
    },
    {
        key: "dmg5",
        chs: "3层引雷伤害",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    return tableThunder(attribute, configObject, enemy, rowsE, "e");
}