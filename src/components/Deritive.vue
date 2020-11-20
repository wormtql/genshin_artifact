<template>
    <div class="root">
        <v-chart :options="options"></v-chart>
        <div>
            <span class="recommend-title">效益最大的属性（降序）：</span>
            <el-tag
                v-for="attr in recommendation"
                :key="attr"
                class="recommend"
            >
                {{ attr }}
            </el-tag>
        </div>
    </div>
</template>

<script>
import ECharts from 'vue-echarts'
import 'echarts/lib/chart/line'
import 'echarts/lib/component/polar'

import { recommendAttribute } from "@/utils";

export default {
    name: "Deritive",
    components: {
        "v-chart": ECharts,
    },
    props: {
        data: {
            type: Array,
            default: () => [
                {
                    "name": "name",
                    "chs": "暴击率",
                    "d": 1000
                },
                {
                    "name": "name1",
                    "chs": "暴击伤害",
                    "d": 900,
                }
            ]
        }
    },
    computed: {
        options() {
            return {
                series: {
                    type: "pie",
                    data: this.data.filter(item => item.d > 0).map(item => ({
                        name: item.chs,
                        value: item.d
                    })),
                }
            }
        },
        recommendation() {
            return recommendAttribute(this.data);
        }
    }
}
</script>

<style scoped>
.root {
    display: flex;
    align-items: center;
    flex-direction: column;
}

.recommend {
    margin: 8px 8px 0 0;
}

.recommend-title {
    font-size: 14px;
    color: #606266;
    /* border: 1px solid gray; */
}
</style>