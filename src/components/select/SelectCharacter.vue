<template>
    <el-select
        :value="value"
        @input="$emit('input', $event)"
        placeholder="角色"
        size="small"
    >
        <el-option-group
            v-for="(group, elementName) in charactersByElement"
            :key="elementName"
            :label="element2Chs(elementName)"
        >
            <el-option
                v-for="character in group"
                :key="character.name"
                :label="character.chs"
                :value="character.name"
            >
                <div class="option-item flex-row">
                    <img :src="character.avatar">
                    <span :style="{ color: getColor(character.star) }">{{ character.chs }}</span>
                </div>
            </el-option>
        </el-option-group>
    </el-select>
</template>

<script>
import { charactersByElement } from "@asset/characters";
import qualityColors from "@const/quality_colors";

export default {
    name: "SelectCharacter",
    props: ["value"],
    created() {
        this.charactersByElement = charactersByElement;
        // console.log(charactersByElement);
    },
    methods: {
        element2Chs(element) {
            switch(element) {
                case "fire":
                    return "火";
                case "ice":
                    return "冰";
                case "grass":
                    return "草";
                case "thunder":
                    return "雷";
                case "wind":
                    return "风";
                case "rock":
                    return "岩";
                case "water":
                    return "水";
            }
        },

        getColor(star) {
            return qualityColors[star - 1];
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