<template>
    <el-select
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
        :placeholder="t('misc.character')"
    >
        <el-option-group
            v-for="(elementName) in elements"
            :key="elementName"
            :label="t('ele', elementName)"
        >
            <el-option
                v-for="(character, index) in characterByElement[elementName]"
                :key="index"
                :label="t('character', character.name)"
                :value="character.name"
            >
                <div class="option-item flex-row">
                    <img :src="character.avatar">
                    <span :style="{ color: getColor(character.star) }">{{ t("character." + character.name) }}</span>
                </div>
            </el-option>
        </el-option-group>
    </el-select>
</template>

<script>
import { characterByElement } from "@character";
import qualityColors from "@const/quality_colors";
import {useI18n} from "@/i18n/i18n";

export default {
    name: "SelectCharacter",
    props: ["modelValue"],
    emits: ["update:modelValue"],
    data() {
        return {
            characterByElement,
            elements: ["Pyro", "Cryo", "Anemo", "Electro", "Hydro", "Geo", "Dendro"]
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
    img {
        width: 28px;
        height: 28px;
        margin-right: 8px;
        border-radius: 50%;
    }
}
</style>