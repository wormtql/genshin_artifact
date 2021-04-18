export default function (attribute, buffs) {
    for (let buff of buffs) {
        switch (buff.type) {
            case "atk-static":
                attribute.attackStatic += buff.value;
                break;
            case "atk-percentage":
                attribute.attackPercentage += buff.value * attribute.attackBasic;
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
        }
    }
}