import { charactersData } from "@asset/characters";
import {TargetFuncUtils} from "@asset/target_functions/utils";


// const skill = charactersData["bachongshenzi"].skill;

function f(attribute, { cArgs }) {
    const hasTalent2 = cArgs.level > 60 || (cArgs.level === 60 && cArgs.ascend);

    if (hasTalent2) {
        let em = attribute.elementalMastery
        attribute.eBonus += em * 0.0015
    }

    let atk = attribute.attack()

    return TargetFuncUtils.damageDefault(attribute, atk, "thunder", "e").expect

    // return dps;
}

export default {
    name: "bachongE",
    func: f,
    needConfig: false,
    needContext: false,
    version: 2,
}