import { artifactsTagMap } from "@asset/artifacts";
import { secondaryTags } from "@asset/tags";

export default function(vm) {
    let head = vm.$store.getters["artifacts/allArtifacts"].head;
    if (vm.headMainStat16Only) {
        head = head.filter(item => {
            return (item.level ?? 20) >= 16;
        })
    }

    let count = {};
    for (let key of artifactsTagMap["head"]) {
        count[key] = 0;
    }
    for (let i = 0; i < head.length; i++) {
        let name = head[i].mainTag.name;
        count[name]++;
    }

    let options = {
        tooltip: {
            trigger: "item"
        },
        series: [
            {
                name: "理之冠主词条分布",
                type: "pie",
                radius: ["40%", "70%"],
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