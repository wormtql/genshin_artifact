<template>
    <div class="item">
        <span class="title">{{ t("misc.lvl") }}</span>
        <el-input-number
            :model-value="modelValue.level"
            @update:modelValue="handleInput('level', $event)"
            :min="60"
            :max="120"
        ></el-input-number>
    </div>

    <div
        v-for="ele in elements8"
        :key="ele"
        class="item"
    >
        <span class="title">{{ t("res", ele) }}</span>
        <el-slider
            :model-value="modelValue[resNameMap[ele]]"
            @update:modelValue="handleInput(ele, $event)"
            class="input"
            :min="-1"
            :max="1"
            :step="0.1"
            :show-input="true"
        ></el-slider>
    </div>
</template>

<script>
import {useI18n} from "@/i18n/i18n";

const resNameMap = {
    Electro: "electro_res",
    Pyro: "pyro_res",
    Hydro: "hydro_res",
    Cryo: "cryo_res",
    Geo: "geo_res",
    Anemo: "anemo_res",
    Dendro: "dendro_res",
    Physical: "physical_res",
}
Object.freeze(resNameMap)

import { elements8 } from "@/constants/misc"

export default {
    name: "EnemyConfig",
    props: ["modelValue"],
    emits: ["update:modelValue"],
    // created() {
    //     this.resNames = resNames
    // },
    data() {
        return {
            elements8,
            resNameMap,
        }
    },
    methods: {
        handleInput(name, value) {
            if (name === "level") {
                let temp = Object.assign({}, this.modelValue)
                temp["level"] = value
                this.$emit("update:modelValue", temp)
            } else {
                const resName = this.resNameMap[name]

                if (this.modelValue[resName] !== value) {
                    let temp = Object.assign({}, this.modelValue)
                    temp[this.resNameMap[name]] = value

                    this.$emit("update:modelValue", temp)
                }
            }
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

<style scoped lang="scss">
$width: 150px;

@media only screen and (min-width: 992px) {
    .item {
        margin-bottom: 12px;
        display: flex;
        align-items: center;
        //width: 100%;

        .title {
            display: inline-block;
            //width: 20%;
            width: $width;
        }

        .input {
            //width: 80%;
            //display: inline;
            //flex-grow: 1;
            width: calc(100% - 150px);
        }

        &:last-of-type {
            margin: 0;
        }
    }
}

@media only screen and (max-width: 992px) {
    .item {
        margin-bottom: 12px;

        .title {
            display: inline-block;
            width: 100%;
            margin-bottom: 4px;
        }

        .input {
            width: 100%;
        }

        &:last-of-type {
            margin: 0;
        }
    }
}

</style>