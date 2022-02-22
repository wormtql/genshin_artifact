import { charactersData } from "@asset/character";


let skill = charactersData["qiqi"].skill;

function f(config) {
    let eLevel = config.cArgs.skill2;
    let bonus = skill.e.cure2[eLevel - 1];
    let s = skill.e.cure2Static[eLevel - 1];

    return function (attribute) {
        let atk = attribute.attack();
        return (atk * bonus + s) * (1 + attribute.cureEffect);
    }
}

export default {
    name: "qiqiE2",
    func: f,
    needConfig: true,
}