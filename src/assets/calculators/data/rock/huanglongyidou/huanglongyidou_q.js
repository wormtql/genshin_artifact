import { getAttribute } from "@util/attribute";
import { charactersData } from "@asset/characters";


let skill = charactersData["huanglongyidou"].skill;

export default function (artifacts, configObject) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    const bonus = attribute.defend() * skill.q.atkBonus[c.skill3 - 1];

    return {
        atkBonus: bonus,
    }
}