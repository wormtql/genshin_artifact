import {
    classTypes,
    elementalBonusMap,
    mainTagRatio,
    normalTagRatio,
} from './scoreRatioConfig';

function f() {
    return function(artifact) {
        if (artifact.level > 7 || artifact.star !== 5) {
            return 0;
        }
        let mainTag = artifact.mainTag.name;
        if (elementalBonusMap[mainTag]) {
            mainTag = elementalBonusMap[mainTag];
        }
        const position = artifact.position;
        let normalTags = [...artifact.normalTags];
        if (normalTags.length === 3) {
            normalTags.push({ name: 'blank', value: 0 });
        }
        const scores = classTypes.map(classType => {
            let score = 0;
            if (mainTagRatio[position][mainTag][classType.name]) {
                score = normalTags.map(normalTag => {
                    return normalTagRatio[normalTag.name][classType.name];
                }).reduce(
                    (previousValue, currentValue) => previousValue +
                        currentValue,
                    score,
                );
                score = classType.ratio *
                    (score + mainTagRatio[position][mainTag][classType.name]);
            }
            return score.toFixed(2);
        });

        return Math.max(...scores);
    };
}

export default {
    name: 'initial',
    func: f,
    needConfig: true,
};
