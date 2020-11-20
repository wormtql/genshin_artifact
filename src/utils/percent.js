import { chsSecondaryTag } from "@/common/chs";

const _percent = new Set([
    "cureEffect",
    "life2",
    "attack2",
    "defend2",
    "critical",
    "criticalDamage",
    "recharge",
    "thunderBonus",
    "fireBonus",
    "waterBonus",
    "iceBonus",
    "windBonus",
    "rockBonus",
    "physicalBonus",
]);

export function toRealValue(tagName, value) {
    if (_percent.has(tagName)) {
        return Number(value) / 100;
    }
    return Number(value);
}

export function displayTag(tagName, value) {
    if (_percent.has(tagName)) {
        let temp = (value * 100).toFixed(1);
        return chsSecondaryTag(tagName) + "：" + String(temp) + "%";
    }

    return chsSecondaryTag(tagName) + "：" + String(value);
}