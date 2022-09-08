<template>
    <v-chart :option="option" style="height: 300px" :autoresize="true"></v-chart>
</template>

<script>
/// #if !USE_CDN
import { use } from "echarts/core"
import { PieChart } from "echarts/charts"
import {
    TooltipComponent,
    TitleComponent,
} from "echarts/components"
import { CanvasRenderer } from "echarts/renderers"

use([
    CanvasRenderer,
    PieChart,
    TooltipComponent,
    TitleComponent,
])
/// #endif
import VChart from "vue-echarts"
import {useI18n} from "../../i18n/i18n";

export default {
    name: "BasicPieChart",
    components: {
        VChart
    },
    props: ["title", "data"],
    // mounted() {
    //     console.log(this.option)
    // },
    computed: {
        option() {
            return {
                title: {
                    text: this.title,
                    left: "center"
                },
                tooltip: {
                    trigger: "item",
                    formatter: "{b}：{d}%",
                },
                // legend: {
                //     orient: 'vertical',
                //     left: 'left'
                // },
                series: [
                    {
                        // name: 'Access From',
                        type: 'pie',
                        radius: '50%',
                        center: ["50%", "60%"],
                        data: this.data,
                        emphasis: {
                            itemStyle: {
                                shadowBlur: 10,
                                shadowOffsetX: 0,
                                shadowColor: 'rgba(0, 0, 0, 0.5)'
                            }
                        }
                    }
                ]
            }
        }
    },
    // setup() {
    //     const { t } = useI18n()
    //
    //     return {
    //         t
    //     }
    // }
}
</script>

<style scoped>

</style>