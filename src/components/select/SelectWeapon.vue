<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
        :placeholder="t('misc.weapon')"
    >
        <el-option
            v-for="weapon in weaponList"
            :key="weapon.name"
            :value="weapon.name"
            :label="t('weapon', weapon.name)"
        >
            <div class="option-item">
                <img :src="weapon.url">
                <span :style="{ color: getColor(weapon.star) }">{{ t("weapon", weapon.name) }}</span>
            </div>
        </el-option>
        
    </el-select>
</template>

<script>
import { weaponByType } from "@weapon";
import qualityColors from "@const/quality_colors";
import {useI18n} from "@/i18n/i18n";

export default {
    name: "SelectWeapon",
    props: {
        type: {
            default: "Bow",
        },
        modelValue: {},
    },
    emits: ["update:modelValue"],
    computed: {
        weaponList() {
            return weaponByType[this.type] ?? []
        }
    },
    methods: {
        getColor(star) {
            return qualityColors[star - 1];
        }
    },
    setup() {
        const { t } = useI18n()

        return {
            t
        }
    }
}
</script>

<style lang="scss" scoped>
.option-item {
    display: flex;
    align-items: center;

    img {
        width: 32px;
        height: 32px;
        margin-right: 8px;
    }

    span {
        font-size: 14px;
    }
}
</style>