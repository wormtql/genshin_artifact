<template>
    <div style="height: 300px">
        <v-chart :option="chartOption"></v-chart>
    </div>
</template>

<script>
import { getValueEff } from "@util/artifacts";
import { secondaryTags } from "@asset/tags";


export default {
    name: "ArtifactsTagEffGraph",
    props: ["artifacts"],
    computed: {
        tagEff() {
            let temp = {};
            if (!this.artifacts) {
                return {};
            }

            for (let artifact of this.artifacts) {
                let star = artifact.star ?? 5;
                if (star <= 3) {
                    continue;
                }
                for (let tag of artifact.normalTags) {
                    let name = tag.name;
                    let value = tag.value;
                    let eff = getValueEff(value, name, star);
                    if (temp[name]) {
                        temp[name] += eff;
                    } else {
                        temp[name] = eff;
                    }
                }
            }

            return temp;
        },

        chartOption() {
            let data = [];
            for (let tagName in this.tagEff) {
                data.push({
                    name: secondaryTags[tagName].chs,
                    value: this.tagEff[tagName],
                });
            }

            return {
                tooltip: {
                    trigger: "item"
                },
                // legend: {
                //     orient: "vertical",
                //     left: "left",
                // },
                series: [
                    {
                        name: "有效词条分布",
                        type: "pie",
                        // radius: ["40%", "70%"],
                        label: {
                            // show: false
                        },
                        itemStyle: {
                            borderRadius: 5,
                            borderColor: '#fff',
                            borderWidth: 2
                        },
                        data,
                    }
                ]
            }
        },
    }
}
</script>