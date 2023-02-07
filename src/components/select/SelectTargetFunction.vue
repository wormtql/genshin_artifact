<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
        :placeholder="t('misc.tf')"
    >
        <el-option-group :label="t('misc.charSpecific')" v-if="characterTargetFunctionList && characterTargetFunctionList.length > 0">
            <el-option
                v-for="item in characterTargetFunctionList"
                :key="item.name"
                :label="ta(item.nameLocale)"
                :value="item.name"
            >
                <div class="option-root">
                    <img class="option-badge" :src="item.badge" />
                    <div class="option-body">
                        <p class="option-title">{{ ta(item.nameLocale) }}</p>
                    </div>
                </div>
            </el-option>
        </el-option-group>

        <el-option-group :label="t('misc.general')" v-if="commonTargetFunctionList">
            <el-option
                v-for="item in commonTargetFunctionList"
                :key="item.name"
                :label="ta(item.nameLocale)"
                :value="item.name"
            >
                <div class="option-root">
                    <img class="option-badge" :src="item.badge" />
                    <div class="option-body">
                        <p class="option-title">{{ ta(item.nameLocale) }}</p>
                    </div>
                </div>
            </el-option>
        </el-option-group>
    </el-select>
</template>

<script>
import { targetFunctionByCharacterName } from "@targetFunction"
import {useI18n} from "@/i18n/i18n";

export default {
    name: "SelectTargetFunction",
    props: {
        modelValue: {},
        characterName: {
            default: "Amber"
        }
    },
    emits: ["update:modelValue"],
    data() {
        return {
            commonTargetFunctionList: targetFunctionByCharacterName["common"]
        }
    },
    // created() {
    //     this.commonTargetFunctionList = targetFunctionByCharacterName["common"]
    // },
    computed: {
        characterTargetFunctionList() {
            return targetFunctionByCharacterName[this.characterName] ?? []
        }
    },
    setup() {
        const { t, ta } = useI18n()

        return {
            t,
            ta
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