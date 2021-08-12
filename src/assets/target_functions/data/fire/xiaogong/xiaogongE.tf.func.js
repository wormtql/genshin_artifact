import reaction from "@/elemental_reaction/reaction_bonus";

let ampFunc = reaction.amp;

function f(config) {
    let vaporize = Math.min(1, config.tArgs.vaporize);
    let melt = Math.min(1, config.tArgs.melt);
    if (melt + vaporize > 1) {
        vaporize = 0;
        melt = 0;
    }
    let normal = 1 - vaporize - melt;


    return function (attribute) {
        const atk = attribute.attack();
        const bonus = attribute.bonus + attribute.fireBonus + attribute.aBonus;

        const em = attribute.elementalMastery;
        const amp = ampFunc(em);

        const crit = Math.min(1, attribute.critical);
        const cd = attribute.criticalDamage;


        let ret
            = atk
            * (1 + crit * cd)
            * (1 + bonus)
            * (normal + (vaporize * 1.5 * (1 + amp + attribute.vaporizeEnhance) + melt * 2 * (1 + amp + attribute.meltEnhance)))

        return ret;
    };
}

export default {
    name: "xiaogongE",
    func: f,
    needConfig: true,
}