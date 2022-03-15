<!-- 选择技能index -->

<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-option-group
            v-for="(list, groupName) in skillMap"
            :key="groupName"
            :label="groupName"
        >
            <el-option
                v-for="item in list"
                :key="item.index"
                :label="item.chs"
                :value="item.index"
            ></el-option>
        </el-option-group>
    </el-select>
</template>

<script>
import { characterData } from "@character"

export default {
    name: "SelectCharacterSkill",
    props: ["characterName", "value"],
    computed: {
        skillMap() {
            const data = characterData[this.characterName]

            let map = {}
            if (data.skillMap1.length > 0) {
                map[data.skillName1] = data.skillMap1
            }
            if (data.skillMap2.length > 0) {
                map[data.skillName2] = data.skillMap2
            }
            if (data.skillMap3.length > 0) {
                map[data.skillName3] = data.skillMap3
            }
            
            return map
        }
    },
    watch: {
        "characterName": function (newValue, oldValue) {
            this.$emit("input", 0)
        }
    }
}
</script>