import * as genshin from "genshin_panel";
import targetFunctionsFunc from "@asset/target_functions/func";
import createCheckFunction from "./create_check_function";
import createFilterFunction from "./create_filter_function";
// import applyBuffs from "./apply_buffs";
import { getAttribute } from "@util/attribute";
import artifactEff from "@const/artifact_eff";

const RECORD_COUNT = 5;

function getArtifactsSetInfo(arts) {
    let temp = {};
    for (let i of arts) {
        if (i) {
            if (temp[i.setName]) {
                temp[i.setName]++;
            } else {
                temp[i.setName] = 1;
            }
        }
    }

    return temp;
}

function checkAttribute(config, attribute) {
    let min = config.constraintAttributeMin;

    if (attribute.attack() < min.attack) {
        return false;
    }
    if (attribute.defend() < min.defend) {
        return false;
    }
    if (attribute.life() < min.life) {
        return false;
    }
    if (attribute.elementalMastery < min.elementalMastery) {
        return false;
    }
    if (attribute.recharge < min.recharge) {
        return false;
    }
    let critical = Math.max(attribute.critical, 0);
    critical = Math.min(critical, 1);
    if (critical < min.critical) {
        return false;
    }
    if (attribute.criticalDamage < min.criticalDamage) {
        return false;
    }

    return true;
}

function computeArtifacts(artifacts, configObject) {
    let {
        character: c,
        weapon: w,
        targetFunc: tf,
        buffs,
        constraint,
        artifactsConfig,
    } = configObject;
    // filter artifacts
    let filterFunc = createFilterFunction(constraint);
    artifacts = filterFunc(artifacts);

    // get character and weapon
    const character = new genshin.Character(c.name, c.level, c.ascend, 0);
    const weapon = new genshin.Weapon(
        w.name,
        w.level,
        w.ascend,
        w.refine,
        w.args
    );

    // construct target function, given name and args
    let targetFunc = targetFunctionsFunc[tf.name];
    const targetFuncVersion = targetFunc.version ?? 1;
    let targetFuncContext = {};
    // if need context, artifacts info will be passed as argument during computing
    const needContext = targetFunc.needContext;

    if (targetFuncVersion === 1) {
        if (targetFunc.needConfig) {
            // the target function need configuration
            targetFunc = targetFunc.func({
                character,
                weapon,
                cArgs: {
                    skill1: c.skill1,
                    skill2: c.skill2,
                    skill3: c.skill3,
                    constellation: c.constellation,
                    level: c.level,
                },
                // target function args
                tArgs: tf.args,
            });
        } else {
            targetFunc = targetFunc.func;
        }
    } else if (targetFuncVersion === 2) {
        targetFuncContext = {
            cArgs: {
                name: character.name,
                skill1: c.skill1,
                skill2: c.skill2,
                skill3: c.skill3,
                constellation: c.constellation,
                level: c.level,
                hasTalent1: character.hasTalent1,
                hasTalent2: character.hasTalent2,
            },
            wArgs: { name: weapon.name, refine: weapon.refine, level: weapon.level },
            tArgs: tf.args,
        };

        targetFunc = targetFunc.func;
    }

    // check(or constraint) function
    const check = createCheckFunction(constraint);

    const flowerCount = Math.max(artifacts.flower.length, 1);
    const featherCount = Math.max(artifacts.feather.length, 1);
    const sandCount = Math.max(artifacts.sand.length, 1);
    const cupCount = Math.max(artifacts.cup.length, 1);
    const headCount = Math.max(artifacts.head.length, 1);

    let maxRecord = [];
    let minIndex = 0;
    for (let floweri = 0; floweri < flowerCount; floweri++) {
        let flower = artifacts.flower[floweri] || null;

        for (let featheri = 0; featheri < featherCount; featheri++) {
            let feather = artifacts.feather[featheri] || null;

            if (!check([flower, feather])) {
                continue;
            }

            for (let sandi = 0; sandi < sandCount; sandi++) {
                let sand = artifacts.sand[sandi] || null;

                if (!check([flower, feather, sand])) {
                    continue;
                }

                for (let cupi = 0; cupi < cupCount; cupi++) {
                    let cup = artifacts.cup[cupi] || null;

                    if (!check([flower, feather, sand, cup])) {
                        continue;
                    }

                    for (let headi = 0; headi < headCount; headi++) {
                        let head = artifacts.head[headi] || null;

                        if (!check([flower, feather, sand, cup, head])) {
                            continue;
                        }

                        let arts = [flower, feather, sand, cup, head].filter(
                            (item) => item
                        );
                        let attribute = getAttribute(arts, c, w, buffs, artifactsConfig);
                        if (!checkAttribute(constraint, attribute)) {
                            continue;
                        }

                        let valueFunction = targetFunc;
                        let context = undefined;
                        if (needContext) {
                            context = {
                                artifactSet: getArtifactsSetInfo([
                                    flower,
                                    feather,
                                    sand,
                                    cup,
                                    head,
                                ]),
                            };
                        }

                        if (targetFuncVersion === 1) {
                            valueFunction = (attr) => targetFunc(attr, context);
                            // if (needContext) {
                            //     value = targetFunc(attribute, context);
                            // } else {
                            //     value = targetFunc(attribute);
                            // }
                        } else if (targetFuncVersion === 2) {
                            valueFunction = (attr) =>
                                targetFunc(attr, targetFuncContext, context);
                        }
                        const value = valueFunction(attribute);

                        if (maxRecord.length < RECORD_COUNT) {
                            maxRecord.push({
                                value,
                                combo: [flower, feather, sand, cup, head],
                                attribute,
                                valueFunction,
                                parameters: [c, w, buffs, artifactsConfig],
                            });
                            if (maxRecord.length === RECORD_COUNT) {
                                minIndex = 0;
                                for (let i = 1; i < maxRecord.length; i++) {
                                    let record = maxRecord[i];
                                    minIndex =
                                        record.value < maxRecord[minIndex].value ? i : minIndex;
                                }
                            }
                        } else if (value > maxRecord[minIndex].value) {
                            maxRecord[minIndex] = {
                                value,
                                combo: [flower, feather, sand, cup, head],
                                attribute,
                                parameters: [c, w, buffs, artifactsConfig],
                                valueFunction,
                            };

                            // determine new min value (arr size very small, no need of heap)
                            minIndex = 0;
                            maxRecord.forEach((record, index, arr) => {
                                minIndex =
                                    record.value < arr[minIndex].value ? index : minIndex;
                            });
                        }

                        // if (value > maxValue) {
                        //     maxCombo = [flower, feather, sand, cup, head];
                        //     maxAttribute = attribute;
                        //     maxValue = value;
                        // }
                    }
                }
            }
        }
    }

    // console.log(maxCombo);

    if (maxRecord.length === 0) {
        return {
            error: {
                reason:
                    "没有符合条件的圣遗物，请仔细检查过滤条件、限定条件（套装等），以及圣遗物导入是否有错误",
                code: 1000,
                isError: true,
            },
        };
    }

    maxRecord.sort((a, b) => {
        return b.value - a.value;
    });

    for (let record of maxRecord) {
        record.howMuchBonusPerTag = calcHowMuchBonusPerTag(record);
        delete record.attribute._lazyList;
        delete record.valueFunction;
    }

    return {
        record: maxRecord,
        error: {
            isError: false,
        },
    };
}

function getNewArts(combo, name, value) {
    const [flower, ...rest] = combo;
    let newFlower = flower;
    const sameTagIndex = flower.normalTags.findIndex((t) => t.name === name);
    if (sameTagIndex >= 0) {
        newFlower = {
            ...flower,
            normalTags: Object.assign([], flower.normalTags, {
                [sameTagIndex]: {
                    name,
                    value: flower.normalTags[sameTagIndex].value + value,
                },
            }),
        };
    } else {
        newFlower = {
            ...flower,
            normalTags: [...flower.normalTags, { name, value }],
        };
    }

    return [newFlower, ...rest];
}

function getNewAttribute(combo, parameters, name) {
    let eff = artifactEff["5"];
    return getAttribute(getNewArts(combo, name, eff[name][3]), ...parameters);
}

function calcHowMuchBonusPerTag(candidate) {
    const { valueFunction, value: baseValue, parameters, combo } = candidate;

    const bonus_S =
        valueFunction(getNewAttribute(combo, parameters, "attackStatic")) /
        baseValue -
        1;
    const bonus_p =
        valueFunction(getNewAttribute(combo, parameters, "attackPercentage")) /
        baseValue -
        1;
    const bonus_c =
        valueFunction(getNewAttribute(combo, parameters, "critical")) / baseValue -
        1;
    const bonus_D =
        valueFunction(getNewAttribute(combo, parameters, "criticalDamage")) /
        baseValue -
        1;
    const bonus_hpp =
        valueFunction(getNewAttribute(combo, parameters, "lifePercentage")) /
        baseValue -
        1;
    const bonus_hps =
        valueFunction(getNewAttribute(combo, parameters, "lifeStatic")) /
        baseValue -
        1;
    const bonus_em =
        valueFunction(getNewAttribute(combo, parameters, "elementalMastery")) /
        baseValue -
        1;
    const bonus_dfp =
        valueFunction(getNewAttribute(combo, parameters, "defendPercentage")) /
        baseValue -
        1;
    const bonus_dfs =
        valueFunction(getNewAttribute(combo, parameters, "defendStatic")) /
        baseValue -
        1;
    const bonus_recharge =
        valueFunction(getNewAttribute(combo, parameters, "recharge")) / baseValue -
        1;

    return {
        bonus_S,
        bonus_p,
        bonus_c,
        bonus_D,
        bonus_hpp,
        bonus_hps,
        bonus_em,
        bonus_dfp,
        bonus_dfs,
        bonus_recharge,
    };
}
// function _test() {
//     let character = new genshin.Character("keqing", 90, false, 0);
//     console.log(character);
// }

export default computeArtifacts;
