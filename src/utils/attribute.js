import * as genshin from "genshin_panel";
import { capitalize } from "./utils";

function getArtifact(myArtifact) {
    let temp = new genshin.ArtifactBuilder()
        .setName(myArtifact.setName)
        .position(myArtifact.position)
        .mainTag(myArtifact.mainTag.name, myArtifact.mainTag.value)
    ;

    for (let tag of myArtifact.normalTags) {
        temp.tag(tag.name, tag.value);
    }

    return temp.build();
}

export function applyBuffs(attribute, buffs) {
    for (let buff of buffs) {
        switch (buff.type) {
            case "atk-static":
                attribute.attackStatic += buff.value;
                break;
            case "atk-percentage":
                attribute.attackPercentage += buff.value * attribute.attackBasic;
                break;
            case "def-static":
                attribute.defendStatic += buff.value;
                break;
            case "def-percentage":
                attribute.defendPercentage += buff.value * attribute.defendBasic;
                break;
            case "bonus":
                attribute.bonus += buff.value;
                break;
            case "critical":
                attribute.crit(buff.value);
                break;
            case "em":
                attribute.elementalMastery += buff.value;
                break;
            case "criticalDamage":
                attribute.criticalDamage += buff.value;
                break;
            case "elementalBonus": {
                let ele = buff.element;
                attribute[ele + "Bonus"] += buff.value;
                break;
            }
            case "qBonus": {
                attribute.qBonus += buff.value;
                break;
            }
            case "reactionEnhance": {
                let reactionTypes = buff.reactionTypes;
                let value = parseFloat(buff.value) ?? 0;

                for (let reactionType of reactionTypes) {
                    switch (reactionType) {
                        case "melt": {
                            attribute.meltEnhance += value;
                            break;
                        }
                        case "vaporize": {
                            attribute.vaporizeEnhance += value;
                            break;
                        }
                        case "overload": {
                            attribute.overloadEnhance += value;
                            break;
                        }
                        case "superconduct": {
                            attribute.superconductEnhance += value;
                            break;
                        }
                        case "electroCharged": {
                            attribute.electroEnhance += value;
                            break;
                        }
                        case "swirl": {
                            attribute.swirlThunderEnhance += value;
                            attribute.swirlFireEnhance += value;
                            attribute.swirlWaterEnhance += value;
                            attribute.swirlIceEnhance += value;
                            break;
                        }
                    }
                }
                break;
            }
            case "enemyDefDown": {
                const value = buff.value ?? 0;
                attribute["enemyDefDown"] += value;
                break;
            }
            case "enemyResDown": {
                const value = buff.value ?? 0;
                const elementTypes = buff.elementTypes ?? [];
                for (let element of elementTypes) {
                    let resAttributeKey = `enemy${capitalize(element)}Down`;
                    attribute[resAttributeKey] += value;
                }
                break;
            }
        }
    }
}

export function getAttribute(artifacts, c, w, stdBuffs, artifactsConfig = {}) {
    let character = new genshin.Character(c.name, c.level, c.ascend, c.constellation, c.args);
    let weapon = new genshin.Weapon(w.name, w.level, w.ascend, w.refine, w.args);

    let builder = new genshin.AttributeBuilder();
    builder
        .character(character)
        .weapon(weapon)
    ;
    for (let art of artifacts) {
        if (art) {
            builder.artifact(getArtifact(art));
        }
    }
    builder.artifactsConfig(artifactsConfig);

    let attribute = builder.build();
    applyBuffs(attribute, stdBuffs);

    return attribute;
}