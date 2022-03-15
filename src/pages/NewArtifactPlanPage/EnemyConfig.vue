<template>
    <div>
        <div class="item">
            <span class="title">等级</span>
            <el-input-number
                :value="value.level"
                @input="handleInput('level', $event)"
                :min="60"
                :max="100"
                size="small"
            ></el-input-number>
        </div>

        <div
            v-for="item in resNames"
            class="item"
        >
            <span class="title">{{ item.title }}</span>
<!--            <el-input-->
<!--                :value="value[item.name]"-->
<!--                size="small"-->
<!--                @input="handleInput(item.name, $event)"-->
<!--                class="input"-->
<!--            ></el-input>-->
            <el-slider
                :value="value[item.name]"
                @input="handleInput(item.name, $event)"
                class="input"
                :min="-1"
                :max="1"
                :step="0.1"
                :show-input="true"
            ></el-slider>
        </div>
    </div>
</template>

<script>
const resNames = [
    { name: "electro_res", title: "雷抗" },
    { name: "pyro_res", title: "火抗" },
    { name: "hydro_res", title: "水抗" },
    { name: "cryo_res", title: "冰抗" },
    { name: "geo_res", title: "岩抗" },
    { name: "anemo_res", title: "风抗" },
    { name: "dendro_res", title: "草抗" },
    { name: "physical_res", title: "物抗" }
]
Object.freeze(resNames)

export default {
    name: "EnemyConfig",
    props: ["value"],
    created() {
        this.resNames = resNames
    },
    methods: {
        handleInput(name, value) {
            let temp = Object.assign({}, this.value)
            temp[name] = value

            this.$emit("input", temp)
        }
    }
}
</script>

<style scoped lang="scss">
$width: 150px;

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
</style>