import { charactersData } from "@asset/characters";


let skill = charactersData["qiqi"].skill;

function f(config) {
    let eLevel = config.cArgs.skill2;
    let bonus = skill.e.cure1[eLevel - 1];
    let s = skill.e.cure1Static[eLevel - 1];

    return function (attribute) {
        let atk = attribute.attack();
        return (atk * bonus + s) * (1 + attribute.cureEffect);
    }
}

export default {
    name: "qiqiE1",
    func: f,
    needConfig: true,
}