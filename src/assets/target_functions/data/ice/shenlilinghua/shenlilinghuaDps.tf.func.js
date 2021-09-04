import { charactersData } from "@asset/characters";

let skill = charactersData["shenlilinghua"].skill;


// function zeroPad(num, places) {
//     var zero = places - num.toString().length + 1;
//     return Array(+(zero > 0 && zero)).join("0") + num;
// }

// var proto = Object.create(Number.prototype);
function PrettyNumber(val) { this.val = val;  this.formatted = ""
}
// PrettyNumber.prototype = proto;
PrettyNumber.prototype.constructor = PrettyNumber;
PrettyNumber.prototype.valueOf = function() { return this.val }
// PrettyNumber.prototype.toFixed = function () {
//     const attr = this.attr
//     // n = n
//     return [
//         // Number.prototype.toFixed.call(this.val, n),
//         this.val.toString(),
//         `百分比攻击力: ${attr.attackPercentage}%`,
//         `爆伤: ${attr.criticalDamage}`,
//         `大招: 间隔 ${attr.energyBurstInterval}s`
//     ].join('\n')
// }

function shenlilinghuaDps(config) {
    // console.log(JSON.parse(JSON.stringify(config.weapon)))
    let hasTalent1 = config.character.hasTalent1;
    let hasTalent2 = config.character.hasTalent2;

    let aLevel = config.cArgs.skill1
    let eLevel = config.cArgs.skill2
    let qLevel = config.cArgs.skill3

    // http://xin.07073.com/zixun/1912038.html
    // 比较实用的技巧是神里可以用闪避取消重击后摇，并用平A取消闪避后摇，一套4A重闪循环也仅仅耗时2.66秒左右，在天赋抵消闪避消耗后成为永动机循环。
    let aDmg = skill.a.dmg1[aLevel] + skill.a.dmg2[aLevel] + skill.a.dmg3[aLevel] + skill.a.dmg4[aLevel] * 3 + skill.a.bDmg1[aLevel] * 3
    let aDmgTime = 2.66
    let aBonus = hasTalent1 ? 0.3 : 0

    // 同时QE也可以用闪避取消后摇，单E耗时1秒整，E接平A耗时0.83秒，E接闪避耗时0.46秒，大招耗时2.16秒，接闪避耗时2.06秒左右(相对来说大招能取消的后摇不明显)。
    let eDmg = skill.e.dmg1[eLevel]
    let eDmgTime = 2.53
    
    let qDmg = skill.q.dmg1[qLevel] * 19 + skill.q.dmg2[qLevel]
    let qDmgTime = 2.16
    let talentBonus = hasTalent2 ? 0.18 : 0 

    let dogeTime = 0.5 // 闪避用时

    let ice4Crit = config.tArgs.ice4Crit;
    let weaponElement = 0

    if(config.weapon.name === "tianmuyingdadao")
    {
        const tianmuEnergyRecharge = [6, 7.5, 9, 10.5, 12]
        weaponElement += tianmuEnergyRecharge[config.weapon.refine] * 1.5 / 30.0
    }

    return function (attribute, context) {
        let attack = attribute.attack();
        let crit = attribute.bCritical;

        let bonus = attribute.bBonus + attribute.bonus + attribute.iceBonus;

        let isBS4 = (context.artifactSet.blizzardStrayer || 0) >= 4;
        if (isBS4) {
            crit += ice4Crit;
        }


        // 元素球充能理论 https://bbs.mihoyo.com/ys/article/1474904
        // 元素球产出: https://bbs.nga.cn/read.php?tid=26932608&rand=468
        // 迪奥纳e长按 3-5 元素球，冷却 15s
        // 神里 e 4-5, cd 10
        
        let helperElement = config.tArgs.doubleIceRecharge ?  4 / 15.0 : 0.0
        let shenliElement = 4.5 / 10.0
        let additionalElement = 4.0 / 20 // 假设一个大招秒两个怪产出微粒

        // 每个同元素微粒能获取约3能量
        let totalElementPerSec = (helperElement + shenliElement + additionalElement) * 2.7
        let energyPerSecond = totalElementPerSec * attribute.recharge + weaponElement
        let energyBurstInterval = Math.max(80 / energyPerSecond, 20)
        
        let critBonus = (1 + Math.min(crit, 1) * attribute.criticalDamage);

        
        let aOutput = aDmg * attack * (1 + bonus + aBonus + talentBonus) * critBonus // aDmgTime
        let eOutput = eDmg * attack * (1 + bonus          + talentBonus) * critBonus // (eDmgTime)
        let qOutput = qDmg * attack * (1 + bonus          + talentBonus) * critBonus // (qDmgTime)

        let timeElapse = 0
        let eTimer = 0
        let qTimer = 0

        let aCount = 0
        let eCount = 0
        let qCount = 0
        
        // 神里三次大招循环，模拟切辅助、普通、e技能以及大招输出流程
        while(qCount < 3)
        {
            let usedTime = 4 + aDmgTime
            timeElapse += usedTime // 辅助一通输出
            eTimer += usedTime
            qTimer += usedTime
            aCount++;

            if(eTimer > 10)
            {
                eCount ++;
                eTimer = 0;
                timeElapse += eDmgTime
            }
            if(qTimer > energyBurstInterval)
            {
                qCount ++;
                qTimer = 0;
                timeElapse += qDmgTime + dogeTime // 闪避之后放大获得冰伤加成
            }
        }

        const totalOutput = (aOutput * aCount + eOutput * eCount + qOutput * qCount)
        const res = totalOutput / timeElapse
        var theRes = new PrettyNumber(res);

        const atkPercentage = (attribute.attackPercentage / attribute.attackBasic * 100).toFixed(1)
        // const greenAtkPercentage = ((attribute.attackPercentage+attribute.attackStatic) / attribute.attackBasic * 100).toFixed(1)
        
        const aPercentage = aOutput * aCount / totalOutput * 100
        const ePercentage = eOutput * eCount / totalOutput * 100
        const qPercentage = qOutput * qCount / totalOutput * 100

        // const critDmgPercentage = (attribute.criticalDamage * 100).toFixed(1)
        theRes.formatted = [
            `期望Dps: ${res.toFixed(1)}`,
            `攻击力增加%: ${atkPercentage}%`, 
            `大招间隔: ${energyBurstInterval.toFixed(1)}s`,
            `输出占比: 普攻(${aPercentage.toFixed(1)}%), 战技(${ePercentage.toFixed(1)}%), 大招(${qPercentage.toFixed(1)}%)`,
            // `输出量 vs 循环中使用次数: ${aOutput.toFixed(1)}:${aCount}, ${eOutput.toFixed(1)}:${eCount}, ${qOutput.toFixed(1)}:${qCount}`,
            // `倍率: ${aDmg.toFixed(1)}, ${eDmg.toFixed(1)}, ${qDmg.toFixed(1)}`
        ].join('  ‏‏‎ ')
        return theRes;
    };
}

export default {
    name: "shenlilinghuaDps",
    func: shenlilinghuaDps,
    needConfig: true,
    needContext: true,
}