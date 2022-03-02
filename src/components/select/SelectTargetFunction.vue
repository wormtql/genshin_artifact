<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        placeholder="目标函数"
        size="small"
    >
        <el-option-group label="角色专属" v-if="characterTargetFunctionList">
            <el-option
                v-for="item in characterTargetFunctionList"
                :key="item.name"
                :label="item.chs"
                :value="item.name"
            >
                <div class="option-root">
                    <img class="option-badge" :src="item.badge" />
                    <div class="option-body">
                        <p class="option-title">{{ item.chs }}</p>
                    </div>
                </div>
            </el-option>
        </el-option-group>

        <el-option-group label="通用" v-if="commonTargetFunctionList">
            <el-option
                v-for="item in commonTargetFunctionList"
                :key="item.name"
                :label="item.chs"
                :value="item.name"
            >
                <div class="option-root">
                    <img class="option-badge" :src="item.badge" />
                    <div class="option-body">
                        <p class="option-title">{{ item.chs }}</p>
                    </div>
                </div>
            </el-option>
        </el-option-group>
    </el-select>
</template>

<script>
import { targetFunctionByCharacterName } from "@targetFunction"

export default {
    name: "SelectTargetFunction",
    props: {
        value: {},
        characterName: {
            default: "Amber"
        }
    },
    created() {
        this.commonTargetFunctionList = targetFunctionByCharacterName["common"]
    },
    computed: {
        characterTargetFunctionList() {
            return targetFunctionByCharacterName[this.characterName]
        }
    },
    watch: {
        "characterName": function (newName, oldName) {
            const temp = targetFunctionByCharacterName[newName]
            if (temp.length > 0) {
                this.$emit("input", temp[0].name)
            }
        }
    }
}
</script>

<style lang="scss" scoped>
.option-root {
    height: 32px;
    display: flex;
    align-items: center;

    .option-badge {
        width: 28px;
        height: 28px;
        border-radius: 50%;
        margin-right: 16px;
    }

    .option-body {
        .option-title {
            margin: 0;
        }
    }
}
</style>