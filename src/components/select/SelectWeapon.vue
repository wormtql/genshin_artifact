<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        placeholder="武器"
        size="small"
    >
        <el-option
            v-for="weapon in weaponTypeMap[type]"
            :key="weapon.name"
            :value="weapon.name"
            :label="weapon.chs"
        >
            <div class="option-item">
                <img :src="weapon.url">
                <span :style="{ color: getColor(weapon.star) }">{{ weapon.chs }}</span>
            </div>
        </el-option>
    </el-select>
</template>

<script>
import { weaponsByType } from "@asset/weapons";
import qualityColors from "@const/quality_colors";

export default {
    name: "SelectWeapon",
    created() {
        this.weaponTypeMap = weaponsByType;
    },
    props: {
        type: {
            default: "any",
        },
        value: {},
    },
    methods: {
        getColor(star) {
            return qualityColors[star - 1];
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