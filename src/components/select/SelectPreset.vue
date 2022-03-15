<template>
    <el-select
        size="small"
        :value="value"
        @input="$emit('input', $event)"
    >
        <el-option
            v-for="item in list"
            :key="item"
            :label="item"
            :value="item"
        ></el-option>
    </el-select>
</template>

<script>
import { mapGetters } from "vuex"

export default {
    name: "SelectPreset",
    props: ["value"],
    computed: {
        ...mapGetters("presets", ["allFlat"]),

        list() {
            let results = []
            for (let preset of this.allFlat) {
                results.push(preset.name)
            }
            return results
        }
    }
}
</script>

<style scoped>

</style>