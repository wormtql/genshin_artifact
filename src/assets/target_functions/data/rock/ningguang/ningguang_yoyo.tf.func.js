import { charactersData } from "@asset/character";


let skill = charactersData["ningguang"].skill;

function ningguang_yoyo(attribute, { tArgs, cArgs }) {
    let skilla = cArgs.skill1 - 1;
    let skillb = cArgs.skill2 - 1;
    let skillc = cArgs.skill3 - 1;

    let isQte = tArgs.isQte; //是否速切
    let atk = attribute.attack();  //攻击力
    let constellation=cArgs.constellation;//命座
    //获取暴击增益
    let aCritBonus =1 + Math.min(1, attribute.critical) * attribute.criticalDamage; 
    let zCritBonus =1 + Math.min(1, attribute.bCritical) * attribute.criticalDamage; 
    let eCritBonus =1 + Math.min(1, attribute.eCritical) * attribute.criticalDamage; 
    let qCritBonus =1 + Math.min(1, attribute.qCritical) * attribute.criticalDamage; 


    //获取增伤增益
    let aBonus = 1 + attribute.bonus + attribute.rockBonus + attribute.aBonus;
    let zBonus = 1 + attribute.bonus + attribute.rockBonus + attribute.bBonus;
    let eBonus = 1 + attribute.bonus + attribute.rockBonus + attribute.eBonus;
    let qBonus = 1 + attribute.bonus + attribute.rockBonus + attribute.qBonus;

    //技能面板
    let a=skill.a.dmg1[skilla]; //普攻倍率
    let aTimes = 6;                     //次数
    let z=skill.a.bDmg1[skilla]; //重击倍率
    let zTimes = 3;
    let xx=skill.a.dmg2[skilla]; //星璇倍率
    let xxTimes = 6;
    let e=skill.e.dmg1[skillb]; //E倍率
    let eTimes = 1;
    let q=skill.q.dmg1[skillc]; //Q倍率
    let qTimes = 12;
    

    //命座提升

    if (constellation>=2){
            eTimes+=1;
            if (constellation>=6){
                xxTimes+=7;
            }
    }


    //速切则减少两轮普攻
    if (isQte) {
        aTimes-=6;
        zTimes-=2;
    }

    let dps
    = atk * a * aTimes * aCritBonus * aBonus
    + atk * z * zTimes * zCritBonus * zBonus
    + atk * xx * xxTimes * zCritBonus * zBonus
    + atk * e * eTimes * eCritBonus * eBonus
    + atk * q * qTimes * qCritBonus * qBonus;

    return dps;
}

export default {
    name: "ningguang_yoyo",
    func: ningguang_yoyo,
    needConfig: false,
    needContext: false,
    version: 2
}

