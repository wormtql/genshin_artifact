<template>
    <div class="buff-config-item">
        <p class="buff-config-title">{{ title }}</p>
        <el-input
            :value="value"
            @input="$emit('input', $event)"
            size="mini"
        >
            <span slot="append" v-if="percentage">%</span>
        </el-input>
    </div>
</template>

<script>
export default {
    name: "SingleValue",
    props: ["title", "value", "percentage"]
}
</script>