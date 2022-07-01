<template>
<!--    <div v-for="row in tableDataForElementUI">-->
<!--        <span v-if="row.chs === '感电'"-->
<!--              style="color: #c250ff"-->
<!--        >{{ row.value.toFixed(1) }}</span>-->
<!--        <span v-if="row.chs === '超载'"-->
<!--              style="color: #ff335a"-->
<!--        >{{ row.value.toFixed(1) }}</span>-->
<!--        <span v-if="row.chs !== '感电' && row.chs !== '超载'">{{ row.value.toFixed(1) }}</span>-->
<!--    </div>-->

    <el-table
        :data="tableDataForElementUI"
    >
        <el-table-column
            prop="chs"
            label="类型"
        ></el-table-column>
        <el-table-column
            label="伤害"
        >
            <template #default="{ row }">
                <template v-if="row && row.value && row.chs">
                    <span v-if="row.chs === '感电'"
                          style="color: #c250ff"
                    >{{ row.value.toFixed(1) }}</span>
                    <span v-if="row.chs === '超载'"
                          style="color: #ff335a"
                    >{{ row.value.toFixed(1) }}</span>
                    <span v-if="row.chs !== '感电' && row.chs !== '超载'">{{ row.value.toFixed(1) }}</span>
                </template>

<!--                <span v-if="row.chs !== '感电' && row.chs !== '超载'">{{ f(row) }}</span>-->
            </template>
        </el-table-column>
    </el-table>
</template>

<script>
export default defineComponent({
    name: "TransformativeDamage",
    props: ["data"],
    computed: {
        tableDataForElementUI() {
            // console.log(this.data)
            let results = []
            results.push({ chs: "感电", value: this.data.electro_charged })
            results.push({ chs: "超载", value: this.data.overload })
            results.push({ chs: "碎冰", value: this.data.shatter })
            results.push({ chs: "超导", value: this.data.superconduct })
            results.push({ chs: "扩散（雷）", value: this.data.swirl_electro })
            results.push({ chs: "扩散（火）", value: this.data.swirl_pyro })
            results.push({ chs: "扩散（冰）", value: this.data.swirl_cryo })
            results.push({ chs: "扩散（水）", value: this.data.swirl_hydro })
            return results
        }
    },
    methods: {
        f(row) {
            console.log(row)
            return row.value.toFixed(1)
        }
    }
    // data() {
    //     return {
    //
    //     }
    // }
})
</script>

<style scoped>

</style>