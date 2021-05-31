<template>
    <div>
        <el-alert
            title="对于未满级的圣遗物，潜力指能够到达的数学期望，对于满级的圣遗物，可以认为是当前评分，具体请参考算法说明"
            :closable="false"
        >
        </el-alert>
        <el-alert
            title="只计算4星和5星圣遗物"
            :closable="false"
            type="warning"
            style="margin-bottom: 20px"
        ></el-alert>
        <div
            class="item hand"
            v-for="(f, key) in potentialFuncData"
            :key="key"
            :class="{ active: key === value }"
            @click="handleClick(key)"
        >
            <div>
                <img class="badge" :src="f.badge">
            </div>
            <div class="detail">
                <p class="title fs-14">{{ f.chs }}</p>
                <div class="description fs-12">
                    <span
                        v-for="(des, index) in f.description"
                        :key="index"
                    >
                        {{ des }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import potentialFuncData from "@asset/potential_functions/data";

export default {
    name: "ChoosePotentialFunc",
    created: function () {
        this.potentialFuncData = potentialFuncData;
    },
    props: ["value"],
    methods: {
        handleClick(name) {
            this.$emit("input", name);
        }
    }
}
</script>

<style lang="scss" scoped>
.item {
    /* height: 64px; */
    padding: 8px;
    transition: 300ms;
    border-radius: 3px;
    display: flex;
    border: 1px solid transparent;

    &:hover {
        background: rgba(0, 0, 0, 0.05);
    }

    .detail {
        padding-left: 16px;

        .title {
            font-weight: bold;
            display: block;
            /* padding: 0; */
            margin: 0;
        }

        .description span {
            display: block;
            color: #555555;
            padding-top: 3px;
        }
    }

    .badge {
        width: 64px;
        display: inline-block;
        vertical-align: top;
    }

    &.active {
        background: #12345611;
        border: 1px solid #12345633;
    }
}
</style>