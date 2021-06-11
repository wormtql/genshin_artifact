<template>
    <el-table
            :data="data"
            size="small"
            stripe
        >
            <el-table-column
                label="反应类型"
                property="title"
            ></el-table-column>
            <el-table-column
                label="伤害"
            >
                <template slot-scope="scope">
                    {{ scope.row.dmg.toFixed(1) }}
                </template>
            </el-table-column>
        </el-table>
</template>

<script>
export default {
    name: "CommonTransformative",
    props: ["data"],
}
</script>