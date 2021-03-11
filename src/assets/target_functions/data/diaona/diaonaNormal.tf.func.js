import skill from "./skill";

function normalDiaona(config) {
    let qLevel = config.cArgs.skill3;
    let bonus = skill.q.hp[qLevel - 1];
    let s = skill.q.s[qLevel - 1];

    return function (attribute, context) {
        let hp = attribute.life();
        let isMB4 = context.artifactSet.maidenBeloved >= 4;
        return (hp * bonus + s) * (1 + attribute.cureEffect + (isMB4 ? 0.2 : 0));
    }
}

export default {
    name: "diaonaNormal",
    func: normalDiaona,
    needConfig: true,
    needContext: true,
}