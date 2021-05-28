import { tableThunder } from "../../../utils";
import { getAttribute } from "@util/attribute";


let skillKeys = [
    {
        key: "dmg1",
        chs: "技能伤害",
    },
    {
        key: "dmg2",
        chs: "连斩伤害",
    },
    {
        key: "dmg3",
        chs: "最后一击",
    },
];

export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let hasTalent2 = (c.level === 60 && c.ascend) || c.level > 60;

    if (hasTalent2) {
        attribute.crit(0.15);
    }

    return tableThunder(attribute, configObject, enemy, skillKeys, "q");
}