<template>
    <el-cascader
        :model-value="modelValue"
        @update:modelValue="handleInput"
        :placeholder="t('misc.character')"
        :options="options"
    >
        <template #default="{ node, data }">
            <div class="option-item flex-row">
                <template v-if="node.isLeaf">
                    <div class="option-item flex-row">
                        <img :src="data.avatar">
                        <span :style="{ color: getColor(data.star) }">{{ t("character." + data.value) }}</span>
                    </div>
<!--                    {{ data.value }}-->
                </template>
                <template v-else>
                    <span>{{ data.label }}</span>
                </template>
<!--                {{ data }}-->
<!--                <img :src="">-->
<!--                <span :style="{ color: getColor() }"></span>-->
            </div>
        </template>
    </el-cascader>
<!--    <el-select-->
<!--        :model-value="modelValue"-->
<!--        @update:modelValue="$emit('update:modelValue', $event)"-->
<!--        :placeholder="t('misc.character')"-->
<!--    >-->
<!--        <el-option-group-->
<!--            v-for="(elementName) in elements"-->
<!--            :key="elementName"-->
<!--            :label="t('ele', elementName)"-->
<!--        >-->
<!--            <el-option-->
<!--                v-for="(character, index) in characterByElement[elementName]"-->
<!--                :key="index"-->
<!--                :label="t('character', character.name)"-->
<!--                :value="character.name"-->
<!--            >-->
<!--                <div class="option-item flex-row">-->
<!--                    <img :src="character.avatar">-->
<!--                    <span :style="{ color: getColor(character.star) }">{{ t("character." + character.name) }}</span>-->
<!--                </div>-->
<!--            </el-option>-->
<!--        </el-option-group>-->
<!--    </el-select>-->
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
        }
    },
    methods: {
        getColor(star) {
            return qualityColors[star - 1];
        },

        handleInput(value) {
            const name = value[1] ?? "Amber"
            this.$emit("update:modelValue", name)
        }
    },
    setup() {
        const { t } = useI18n()

        const elements = ["Pyro", "Cryo", "Anemo", "Electro", "Hydro", "Geo", "Dendro"]

        const options = computed(() => {
            const result = []
            for (const element of elements) {
                const locale = t("ele", element)

                const characters = []
                for (const character of characterByElement[element]) {
                    const characterName = character.name
                    const characterNameLocale = t("character", characterName)
                    characters.push({
                        value: characterName,
                        label: characterNameLocale,
                        avatar: character.avatar,
                        star: character.star,
                    })
                }

                result.push({
                    value: element,
                    label: locale,
                    children: characters
                })
            }
            return result
        })

        return {
            t,
            options,
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