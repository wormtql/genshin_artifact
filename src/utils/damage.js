import { damageDelegate, damageReactionDelegate } from "../assets/calculators/utils";
import Enemy from "@asset/enemies/enemy"

const enemy = new Enemy("hilichurl", 80)

export function damageDefault(attribute, dmg, element, skill) {
    return damageDelegate(attribute, 90, dmg, enemy, element, skill)
}

export function damageReactionDefault(attribute, dmg, element, skill, reaction) {
    const baseDamage = damageDefault(attribute, dmg, element, skill)
    const reactionDamage = damageReactionDelegate(reaction, attribute, baseDamage, element)

    return reactionDamage
}