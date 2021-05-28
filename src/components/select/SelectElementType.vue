<template>
    <el-radio-group
        :value="value"
        @input="$emit('input', $event)"
        size="small"
    >
        <el-radio-button v-if="eleMap.has('fire')" label="fire">火</el-radio-button>
        <el-radio-button v-if="eleMap.has('thunder')" label="thunder">雷</el-radio-button>
        <el-radio-button v-if="eleMap.has('water')" label="water">水</el-radio-button>
        <el-radio-button v-if="eleMap.has('wind')" label="wind">风</el-radio-button>
        <el-radio-button v-if="eleMap.has('rock')" label="rock">岩</el-radio-button>
        <el-radio-button v-if="eleMap.has('ice')" label="ice">冰</el-radio-button>
        <el-radio-button v-if="eleMap.has('physical')" label="physical">物理</el-radio-button>
    </el-radio-group>
</template>

<script>
export default {
    name: "SelectElementType",
    props: {
        value: {
            type: String,
            required: true,
        },
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