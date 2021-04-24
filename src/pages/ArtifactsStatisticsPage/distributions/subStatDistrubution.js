import artifactEff from "@const/artifact_eff";
import { secondaryTags } from "@asset/tags";
import { howManyUpgradeCount } from "@util/artifacts";

export default function(vm) {
    let allFlat = vm.$store.getters["artifacts/allFlat"];

    // allFlat = allFlat.filter(item => {
    //     let level = item.level ?? 20;
    //     return level >= vm.subStatEffLevelMin && level <= vm.subStatEffLevelMax;
    // });
    // if (vm.subStatEff5StarOnly) {
    //     allFlat = allFlat.filter(item => (item.star ?? 5) === 5);
    // }

    let count = {};
    for (let key in artifactEff[5]) {
        count[key] = 0;
    }
    
    for (let i = 0; i < allFlat.length; i++) {
        let art = allFlat[i];
        let star = art.star ?? 5;
        if (star <= 3) {
            continue;
        }
        let effStar = artifactEff[star];
        for (let j = 0; j < art.normalTags.length; j++) {
            let tag = art.normalTags[j];

            let value = howManyUpgradeCount(tag.value, effStar[tag.name]);
            count[tag.name] += value;
        }
    }

    let sum = Object.values(count).reduce((a, b) => a + b);

    let options = {
        tooltip: {
            trigger: "item",
        },
        legend: {
            // top: "0",
            left: "left",
            orient: "vertical",
        },
        series: [
            {
                name: "副词条效率分布",
                type: "pie",
                radius: ["40%", "70%"],
                label: {
                    // show: false,
                    formatter: ({ data }) => {
                        if (sum === 0) {
                            return "没有满足条件的圣遗物";
                        }
                        return `${data.name}: ${(data.value * 100 / sum).toFixed(3)}%`
                    }
                },
                itemStyle: {
                    borderRadius: 10,
                    borderColor: '#fff',
                    borderWidth: 2
                },
                data: Object.keys(count).map(key => ({
                    name: secondaryTags[key].chs,
                    value: count[key]
                })).filter(item => item.value > 0),
            }
        ]
    }

    return options;
}