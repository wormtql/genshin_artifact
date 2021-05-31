import positions from "@const/positions";
import artifactEff from "@const/artifact_eff";

export function count(artifacts) {
    let c = 0;

    positions.forEach(pos => {
        c += artifacts[pos].length;
    });

    return c;
}

export function howManyUpgradeCount(value, tagName, star) {
    let eff = artifactEff[star][tagName];
    if (tagName === "attackPercentage") {
        console.log(value);
    }

    value = Math.round(value * 1000);
    let eff0 = Math.round(eff[0] * 1000);
    let eff3 = Math.round(eff[3] * 1000);

    let max = Math.floor(value / eff0);
    let min = Math.ceil(value / eff3);
    return [min, max];
}

export function getValueEff(value, tagName, star) {
    let eff = artifactEff[star][tagName];

    return value / eff[3];
}