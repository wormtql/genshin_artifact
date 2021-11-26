import { damageDelegate } from "../assets/calculators/utils";
import Enemy from "@asset/enemies/enemy"

const enemy = new Enemy("hilichurl", 80)

export function damageDefault(attribute, dmg, element, skill) {
    return damageDelegate(attribute, 90, dmg, enemy, element, skill)
}