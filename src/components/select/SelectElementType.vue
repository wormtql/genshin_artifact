<template>
    <el-radio-group
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-radio-button v-if="eleMap.has('Pyro')" label="Pyro">{{ t("ele.Pyro") }}</el-radio-button>
        <el-radio-button v-if="eleMap.has('Electro')" label="Electro">{{ t("ele.Electro") }}</el-radio-button>
        <el-radio-button v-if="eleMap.has('Hydro')" label="Hydro">{{ t("ele.Hydro") }}</el-radio-button>
        <el-radio-button v-if="eleMap.has('Anemo')" label="Anemo">{{ t("ele.Anemo") }}</el-radio-button>
        <el-radio-button v-if="eleMap.has('Geo')" label="Geo">{{ t("ele.Geo") }}</el-radio-button>
        <el-radio-button v-if="eleMap.has('Cryo')" label="Cryo">{{ t("ele.Cryo") }}</el-radio-button>
        <el-radio-button v-if="eleMap.has('Dendro')" label="Dendro">{{ t("ele.Dendro") }}</el-radio-button>
        <el-radio-button v-if="eleMap.has('Physical')" label="Physical">{{ t("ele.Physical") }}</el-radio-button>
    </el-radio-group>
</template>

<script>
import {useI18n} from "../../i18n/i18n"

export default {
    name: "SelectElementType",
    emits: ["update:modelValue"],
    props: {
        modelValue: {
            type: String,
            required: true,
        },
        elements: {
            default: () => {
                return ["Pyro", "Electro", "Hydro", "Anemo", "Geo", "Cryo", "Dendro", "Physical"];
            }
        },
    },
    computed: {
        eleMap() {
            let temp = new Set();
            for (let ele of this.elements) {
                temp.add(ele);
            }
            return temp;
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