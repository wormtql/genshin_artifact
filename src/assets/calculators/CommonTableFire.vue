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
                label="火元素伤害"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.fire"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="融化"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.fireMelt"></damage-display>
                </template>
            </el-table-column>
            <el-table-column
                label="蒸发"
                width="200"
            >
                <template slot-scope="scope">
                    <damage-display :damage="scope.row.fireVaporize"></damage-display>
                </template>
            </el-table-column>
        </el-table>
</template>

<script>
import DamageDisplay from "@c/display/DamageDisplay";

export default {
    name: "CommonTableFire",
    components: {
        DamageDisplay,
    },
    props: ["data"],
}
</script>