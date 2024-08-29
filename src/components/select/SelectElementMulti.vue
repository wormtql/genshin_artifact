<template>
    <el-checkbox-group
        :model-value="modelValue"
        @update:modelValue="$emit('update:modelValue', $event)"
    >
        <el-checkbox-button v-if="eleMap.has('Pyro')" label="Pyro">火</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('Electro')" label="Electro">雷</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('Hydro')" label="Hydro">水</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('Anemo')" label="Anemo">风</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('Geo')" label="Geo">岩</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('Cryo')" label="Cryo">冰</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('Dendro')" label="Dendro">草</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('Physical')" label="Physical">物理</el-checkbox-button>
    </el-checkbox-group>
</template>

<script>
const indexMap = {
    "Electro": 0,
    "Pyro": 1,
    "Cryo": 2,
    "Dendro": 3,
    "Geo": 4,
    "Anemo": 5,
    "Hydro": 6,
    "Physical": 7
}
const indexMapReverse = ["Electro", "Pyro", "Cryo", "Dendro", "Geo", "Anemo", "Hydro", "Physical"]

export default {
    name: "SelectElementMulti",
    emits: ["update:modelValue"],
    props: {
        modelValue: {
            type: Array,
            default: () => {
                return [false, false, false, false, false, false, false, false]
            }
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
        },
    }
}
</script>
