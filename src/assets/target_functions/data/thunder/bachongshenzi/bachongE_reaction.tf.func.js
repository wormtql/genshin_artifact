import { charactersData } from "@asset/characters";
import {TargetFuncUtils} from "@asset/target_functions/utils";


const skill = charactersData["bachongshenzi"].skill;

function f(attribute, { tArgs,cArgs }) {
    const hasTalent2 = cArgs.level > 60 || (cArgs.level === 60 && cArgs.ascend);

    let atk = attribute.attack();
    let eSkill = cArgs.skill2-1;
    let ePercent = skill.e.dmg3[eSkill];
    let constellation=cArgs.constellation;
    let eTimes = 15 //E总攻击次数
    let defRadio = 0.5 //预设防御系数

    //命座2升阶
    if (constellation>=2){
        ePercent = skill.e.dmg4[eSkill]
    }

    //获取暴击增益
    let eCritBonus =1 + Math.min(1, attribute.eCritical) * attribute.criticalDamage; 

    //获取增伤增益
    let eBonus = 1 + attribute.bonus + attribute.thunderBonus + attribute.eBonus;

    if (hasTalent2) {
        let em = attribute.elementalMastery;
        eBonus += em * 0.0015;
    }

    let cSixBonus = 0;

       //4命减防提升  经过测试，计算器本身并不会给自动加上4命的20%雷伤，这里给加伤了。
       if (constellation>=4) {
        eBonus += 0.2;
    }

    //6命减防提升
    if (constellation>=6) {
        cSixBonus = 0.428;
    }



    let mode = tArgs.mode;
    let reation_baseDamge = 0.0;
    let reationTimes = 0.0;

    if (mode == 'ElectroCharged'){
        reation_baseDamge = 1736;
        reationTimes =tArgs.electroChargedTimes;
        
    }else{
        reation_baseDamge = 2894;
        reationTimes =tArgs.Overloadedtimes;
    }

    let reationsBous = 1+16*attribute.elementalMastery/(attribute.elementalMastery+2000)

    let dps = 
      atk *ePercent* eCritBonus * eBonus * eTimes * defRadio * ( 1 + cSixBonus )
    + reation_baseDamge*reationsBous*reationTimes;

    return dps;
}

export default {
    name: "bachongE_reaction",
    func: f,
    needConfig: true,
    needContext: false,
    version: 2,
}