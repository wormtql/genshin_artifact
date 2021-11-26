import { TargetFuncUtils } from "../../../utils";


function f(attribute) {
    const def = attribute.defend();
    
    const dmg = TargetFuncUtils.damageDefault(attribute, def, "rock", "e").expect;
    return dmg;
}

export default {
    name: "abeiduoE",
    needConfig: false,
    needContext: false,
    func: f,
}