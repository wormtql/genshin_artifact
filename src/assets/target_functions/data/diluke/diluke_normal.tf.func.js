import reaction from "@/elemental_reaction/reaction_bonus";

let ampFunc = reaction.amp;

function f(config) {
    let aFreq = config.tArgs.aFreq;
    let bFreq = config.tArgs.bFreq;
    let eFreq = config.tArgs.eFreq;
    let qFreq = config.tArgs.qFreq;

    let fireFreq = config.tArgs.fireFreq;

    let evaporate = config.tArgs.evaporate;
    let melt = config.tArgs.melt;
    let normalFreq = 1 - evaporate - melt;

    return function (attribute, context) {
        let isCW4 = (context.artifactSet.crimsonWitch ?? 0) >= 4;
        let isLW4 = (context.artifactSet.lavaWalker ?? 0) >= 4;

        let otherBonus = isLW4 ? (fireFreq * 0.35) : 0;

        const attack = attribute.attack();
        const critical = Math.min(attribute.critical, 1);
        const bCritical = Math.min(attribute.bCritical, 1);
        const eCritical = Math.min(attribute.eCritical, 1);
        const qCritical = Math.min(attribute.qCritical, 1);

        let commonBonus = attribute.fireBonus + attribute.bonus + otherBonus;
        let commonBonus2 = attribute.physicalBonus + attribute.bonus + otherBonus;

        let em = attribute.elementalMastery;
        let amp = ampFunc(em);
        if (isCW4) {
            commonBonus += 0.075;
            amp += 0.15;
        }

        let a
            = (1 + attribute.aBonus + commonBonus2)
            * (critical * attribute.criticalDamage + 1)
            * 1.44
            ;

        let b
            = (1 + attribute.bBonus + commonBonus2)
            * (bCritical * attribute.criticalDamage + 1)
            * 1
            ;

        let e
            = (1 + attribute.eBonus + commonBonus)
            * (eCritical * attribute.criticalDamage + 1)
            * (normalFreq + (evaporate * 1.5 + melt * 2) * (1 + amp))   // reaction rate
            * 1.5;

        let q
            = (1 + attribute.qBonus + commonBonus)
            * (qCritical * attribute.criticalDamage + 1)
            * (normalFreq + (evaporate * 1.5 + melt * 2) * (1 + amp))   // reaction rate
            * 6.558;

        let expect = (a * aFreq + b * bFreq + e * eFreq + q * qFreq) * attack;

        return expect;
    }
}

export default {
    name: "dilukeNormal",
    func: f,
    needConfig: true,
    needContext: true,
}