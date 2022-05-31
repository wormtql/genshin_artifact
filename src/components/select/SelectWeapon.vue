<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        placeholder="武器"
        size="small"
    >
        <el-option
            v-for="weapon in weaponList"
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
import { weaponByType } from "@weapon";
import qualityColors from "@const/quality_colors";

export default {
    name: "SelectWeapon",
    props: {
        type: {
            default: "Bow",
        },
        value: {},
    },
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
    watch: {
        // "type": function (newWeaponType, oldWeaponType) {
        //     const defaultWeaponData = weaponByType[newWeaponType][0]
        //     const name = defaultWeaponData.name
        //     this.$emit("input", name)
        // }
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