<template>
    <div style="min-width: 100px">
        <div class="header" :title="result">{{ title }}</div>
        <div>
            <div
                v-for="item in arr"
                :key="item.name"
                class="item"
            >
                <div class="item-top">
                    <el-checkbox v-model="item.checked">{{ item.name }}</el-checkbox>
<!--                    <span class="name">{{ item.name }}</span>-->
                </div>
                <div class="item-bottom">
                    <input type="text" v-model.number="item.value" class="input" />
                </div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: "DamageAnalysisUtil",
    props: {
        arr: {},
        title: {
            default: "title"
        },
    },
    computed: {
        result() {
            let s = 0
            for (let item of this.arr) {
                if (item.checked) {
                    s += parseFloat(item.value)
                }
            }
            return Math.round(s*1000)/1000
        }
    }
}
</script>

<style lang="scss" scoped>
.header {
    display: flex;
    height: 32px;
    justify-content: center;
    align-items: center;
    background: rgb(236, 245, 255);
}

.item {
    padding: 4px;
    //border-bottom: 1px solid #00000011;

    .item-top {
        display: flex;
        align-items: center;
        margin-bottom: 4px;
    }

    .name {
        margin: 0 4px;
    }

    .input {
        border: none;
        outline: none;
        width: 100%;
        box-sizing: border-box;
        padding: 4px;
        transition: 200ms;
        border-bottom: 1px solid transparent;
        //transform-origin: left;

        &:focus {
            //background-color: #00000005;
            border-bottom: 1px solid #409EFF;
        }
    }
}
</style>