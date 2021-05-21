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
                label="冰元素伤害"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.ice"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="融化"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.iceMelt"></damage-display>
                </template>
            </el-table-column>
        </el-table>
</template>

<script>
import DamageDisplay from "@c/display/DamageDisplay";

export default {
    name: "CommonTableIce",
    components: {
        DamageDisplay,
    },
    props: ["data"],
}
</script>