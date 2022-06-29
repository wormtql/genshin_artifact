<template>
    <el-radio-group
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-radio-button v-if="eleMap.has('Pyro')" label="Pyro">火</el-radio-button>
        <el-radio-button v-if="eleMap.has('Electro')" label="Electro">雷</el-radio-button>
        <el-radio-button v-if="eleMap.has('Hydro')" label="Hydro">水</el-radio-button>
        <el-radio-button v-if="eleMap.has('Anemo')" label="Anemo">风</el-radio-button>
        <el-radio-button v-if="eleMap.has('Geo')" label="Geo">岩</el-radio-button>
        <el-radio-button v-if="eleMap.has('Cryo')" label="Cryo">冰</el-radio-button>
        <el-radio-button v-if="eleMap.has('Dendro')" label="Dendro">草</el-radio-button>
        <el-radio-button v-if="eleMap.has('Physical')" label="Physical">物理</el-radio-button>
    </el-radio-group>
</template>

<script>
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
    }
}
</script>