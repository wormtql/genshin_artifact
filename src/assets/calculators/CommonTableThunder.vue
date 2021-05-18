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
                label="雷元素伤害"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.thunder"></damage-display>
                </template>
            </el-table-column>
        </el-table>
</template>

<script>
import DamageDisplay from "@c/display/DamageDisplay";

export default {
    name: "CommonTableThunder",
    components: {
        DamageDisplay,
    },
    props: ["data"],
}
</script>