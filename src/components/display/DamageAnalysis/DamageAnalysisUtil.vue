<template>
    <div style="width: 100%">
        <div class="header">{{ title }}</div>
        <div>
            <div
                v-for="item in arr"
                :key="item.name"
                class="item"
            >
                <div class="item-top">
                    <el-checkbox v-model="item.checked"></el-checkbox>
                    <span class="name">{{ item.name }}</span>
                </div>
                <div class="item-bottom">
                    <input type="text" v-model="item.value" class="input" />
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
    }
}
</script>

<style lang="scss" scoped>
.header {
    display: flex;
    height: 32px;
    justify-content: center;
    align-items: center;
    background: #00000011;
}

.item {
    padding: 4px;
    border-bottom: 1px solid #00000011;

    .item-top {
        display: flex;
        align-items: center;
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

        &:focus {
            background-color: #00000011;
        }
    }
}
</style>