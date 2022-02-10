import { charactersData } from "@asset/characters";


let skill = charactersData["keli"].skill;

function keli_yoyo(attribute, {  cArgs }) {
    let skilla = cArgs.skill1 - 1;
    let skillb = cArgs.skill2 - 1;
    let skillc = cArgs.skill3 - 1;

    //let isQte = tArgs.isQte; //是否速切
    let atk = attribute.attack();  //攻击力
    let constellation=cArgs.constellation;//命座
    //获取暴击增益
    let aCritBonus =1 + Math.min(1, attribute.critical) * attribute.criticalDamage; 
    let zCritBonus =1 + Math.min(1, attribute.bCritical) * attribute.criticalDamage; 
    let eCritBonus =1 + Math.min(1, attribute.eCritical) * attribute.criticalDamage; 
    let qCritBonus =1 + Math.min(1, attribute.qCritical) * attribute.criticalDamage; 

    //获取增伤增益
    let aBonus = 1 + attribute.bonus + attribute.fireBonus + attribute.aBonus;
    let zBonus = 1 + attribute.bonus + attribute.fireBonus + attribute.bBonus;
    let eBonus = 1 + attribute.bonus + attribute.fireBonus + attribute.eBonus;
    let qBonus = 1 + attribute.bonus + attribute.fireBonus + attribute.qBonus;

    //技能面板
    let a=skill.a.dmg1[skilla]+skill.a.dmg2[skilla]; //普攻倍率
    let aTimes = 4;                     //次数
    let z=skill.a.bDmg1[skilla]; //重击倍率
    let zTimes = 2;                     //次数
    let zexTimes = 2;                     //强化次数
    let hh=skill.q.dmg1[skillc]*1.2; //1命火花倍率
    let hhTimes = 0;                     //次数
    let e=skill.e.dmg1[skillb]*3+skill.e.dmg2[skillb]*8; //E倍率
    let eTimes = 2;
    let q=skill.q.dmg1[skillc]; //Q倍率
    let qTimes = 20;
    let qex=5.55; //4命爆炸倍率
    let qexTimes = 0;
    

    //命座提升

    if (constellation>=1){
        hhTimes+=4;
            if (constellation>=4){
                qexTimes+=1;
            }
    }


    //速切则减少两轮普攻
    /*   if (isQte) {

    }
    */
    let dps
    = atk * a * aTimes * aCritBonus * aBonus
    + atk * z * zTimes * zCritBonus * zBonus
    + atk * z * zexTimes * zCritBonus * (zBonus+0.5)
    + atk * e * eTimes * eCritBonus * eBonus
    + atk * q * qTimes * qCritBonus * qBonus
    + atk * hh * hhTimes * qCritBonus * qBonus
    + atk * qex * qexTimes * qCritBonus * qBonus;
/*
    console.log("===========分隔符=============");
    console.log("普攻伤害是",atk * a * aTimes * aCritBonus * aBonus);
    console.log("重击伤害是",atk * z * zTimes * zCritBonus * zBonus);
    console.log("强化重击伤害是",atk * z * zexTimes * zCritBonus * (zBonus+0.5));
    console.log("E伤害是",atk * e * eTimes * eCritBonus * eBonus);
    console.log("Q伤害是",atk * q * qTimes * qCritBonus * qBonus);
    console.log("1命伤害是",atk * hh * hhTimes * qCritBonus * qBonus);
    console.log("4命伤害是",atk * qex * qexTimes * qCritBonus * qBonus);
    console.log("============分隔符============");
*/


/*
    console.log("===========分隔符=============");
    console.log("普攻倍率是",a * aTimes);
    console.log("重击倍率是",z * zTimes);
    console.log("强化强化重击倍率是",z * zexTimes);
    console.log("E倍率是",e * eTimes);
    console.log("Q倍率是",q * qTimes);
    console.log("1命倍率是",hh * hhTimes);
    console.log("4命倍率是",qex * qexTimes);
    console.log("============分隔符============");
    */

    return dps;
}

export default {
    name: "keli_yoyo",
    func: keli_yoyo,
    needConfig: false,
    needContext: false,
    version: 2
}

