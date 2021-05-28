<template>
    <el-checkbox-group
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-checkbox-button v-if="eleMap.has('fire')" label="fire">火</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('thunder')" label="thunder">雷</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('water')" label="water">水</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('wind')" label="wind">风</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('rock')" label="rock">岩</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('ice')" label="ice">冰</el-checkbox-button>
        <el-checkbox-button v-if="eleMap.has('physical')" label="physical">物理</el-checkbox-button>
    </el-checkbox-group>
</template>

<script>
export default {
    name: "SelectElementMulti",
    props: {
        value: {},
        elements: {
            default: () => {
                return ["fire", "thunder", "water", "wind", "rock", "ice", "physical"];
            }
        },
        multi: { default: false }
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