import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";

let hutaoSkill = charactersData["hutao"].skill;

export default function (artifacts, configObject) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs);

    let ret = [{
        chs: "治疗量",
        cure: hutaoSkill.q.cure1[c.skill3 - 1] * attribute.life(),
    }, {
        chs: "低血量治疗量",
        cure: hutaoSkill.q.cure2[c.skill3 - 1] * attribute.life(),
    }]

    return ret;
}