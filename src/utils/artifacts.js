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
    
    let max = Math.floor(value / eff[0]);
    let min = Math.ceil(value / eff[3]);
    return [min, max];
}