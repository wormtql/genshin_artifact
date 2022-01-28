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
            const items = characterData[this.characterName]?.skillMap ?? []
            let temp = {}
            for (const item of items) {
                const groupName = item.group
                if (!Object.prototype.hasOwnProperty.call(temp, groupName)) {
                    temp[groupName] = []
                }
                temp[groupName].push(item)
            }
            return temp
        }
    },
    watch: {
        "characterName": function (newValue, oldValue) {
            this.$emit("input", 0)
        }
    }
}
</script>