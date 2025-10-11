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
            :label="t('misc.type1')"
        >
            <template #default="{ row }">
                {{ t("dmg", row.key) }}
            </template>
        </el-table-column>
        <el-table-column
            :label="t('misc.dmg')"
        >
            <template #default="{ row }">
                <template v-if="row && row.key && row.value">
                    <span v-if="row.key === 'electroCharged'"
                          style="color: #c250ff"
                    >{{ row.value.toFixed(1) }}</span>
                    <span v-else-if="row.key === 'overload'"
                          style="color: #ff335a"
                    >{{ row.value.toFixed(1) }}</span>
                    <span v-else>{{ row.value.toFixed(1) }}</span>
                </template>
            </template>
        </el-table-column>
    </el-table>
</template>

<script>
import {useI18n} from "@/i18n/i18n";

export default defineComponent({
    name: "TransformativeDamage",
    props: ["data"],
    computed: {
        tableDataForElementUI() {
            // console.log(this.data)
            let results = []
            results.push({ value: this.data.hyperbloom, key: "hyperbloom" })
            results.push({ value: this.data.burgeon, key: "burgeon" })
            results.push({ value: this.data.bloom, key: "bloom" })
            results.push({ value: this.data.electro_charged, key: "electroCharged" })
            results.push({ value: this.data.overload, key: "overload" })
            results.push({ value: this.data.shatter, key: "shattered" })
            results.push({ value: this.data.superconduct, key: "superConduct" })
            results.push({ value: this.data.burning, key: "burning" })
            results.push({ value: this.data.swirl_electro, key: "swirlElectro" })
            results.push({ value: this.data.swirl_pyro, key: "swirlPyro" })
            results.push({ value: this.data.swirl_cryo, key: "swirlCryo" })
            results.push({ value: this.data.swirl_hydro, key: "swirlHydro" })
            results.push({ value: this.data.crystallize, key: "crystallize" })
            
            // 添加月感电和月绽放反应（如果存在数据）
            if (this.data.moonelectro !== undefined) {
                results.push({ value: this.data.moonelectro, key: "moonelectro" })
            }
            if (this.data.direct_moonelectro !== undefined) {
                results.push({ value: this.data.direct_moonelectro, key: "direct_moonelectro" })
            }
            if (this.data.moonfall !== undefined) {
                results.push({ value: this.data.moonfall, key: "moonfall" })
            }
            
            return results
        }
    },
    methods: {
        f(row) {
            console.log(row)
            return row.value.toFixed(1)
        }
    },
    setup() {
        const { t } = useI18n()

        return {
            t
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