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
                        <span :style="{ color: getColor(data.star) }">{{ data.label }}</span>
                    </div>
                </template>
                <template v-else>
                    <span>{{ data.label }}</span>
                </template>
            </div>
        </template>
    </el-cascader>
</template>

<script>
import { characterByElement, characterData } from "@character";
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
        const { t, ta } = useI18n()

        const elements = ["Pyro", "Cryo", "Anemo", "Electro", "Hydro", "Geo", "Dendro"]

        const options = computed(() => {
            const result = []
            for (const element of elements) {
                const locale = t("ele", element)

                const characters = []
                for (const character of characterByElement[element]) {
                    const characterName = character.name
                    const characterNameLocale = ta(characterData[characterName].nameLocale)
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
            ta,
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