<template>
    <el-table
            :data="data"
            size="small"
            stripe
        >
            <el-table-column
                label="技能"
                property="chs"
                width="150"
            ></el-table-column>
            <el-table-column
                label="水元素伤害"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display
                        :damage="scope.row.water"
                        :show-expect="getShowExpect(scope.row)"
                        :show-critical="getShowCritical(scope.row)"
                        :show-non-critical="getShowNonCritical(scope.row)"
                    ></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="蒸发伤害"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display
                        :damage="scope.row.waterVaporize"
                        :show-expect="getShowExpect(scope.row)"
                        :show-critical="getShowCritical(scope.row)"
                        :show-non-critical="getShowNonCritical(scope.row)"
                    ></damage-display>
                </template>
            </el-table-column>
        </el-table>
</template>

<script>
import DamageDisplay from "@c/display/DamageDisplay";

export default {
    name: "CommonTableWater",
    components: {
        DamageDisplay,
    },
    props: ["data"],
    methods: {
        getShowExpect(row) {
            if (!row.tag) {
                return true;
            }

            if (row.tag === "expectOnly") {
                return true;
            }

            return true;
        },

        getShowCritical(row) {
            if (!row.tag) {
                return true;
            }

            if (row.tag === "expectOnly") {
                return false;
            }

            return true;
        },

        getShowNonCritical(row) {
            if (!row.tag) {
                return true;
            }

            if (row.tag === "expectOnly") {
                return false;
            }

            return true;
        }
    }
}
</script>