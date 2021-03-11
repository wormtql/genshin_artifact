<template>
    <div>
        <el-alert
            title="对于未满级的圣遗物，潜力指能够到达的数学期望，对于满级的圣遗物，可以认为是当前评分，具体请参考算法说明"
            :closable="false"
        >
        </el-alert>
        <el-alert
            title="目前对非5星圣遗物支持不好，因此，尽量使用5星圣遗物；未指定等级和星级的默认为5星20级"
            :closable="false"
            type="warning"
            style="margin-bottom: 20px"
        ></el-alert>
        <div
            class="item hand"
            v-for="(f, key) in potentialFuncData"
            :key="key"
            @click="handleClick(f.name)"
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
    methods: {
        handleClick(name) {
            this.$emit("select", name);
        }
    }
}
</script>

<style scoped>
.detail {
    padding-left: 16px;
}

.item {
    /* height: 64px; */
    padding: 8px;
    transition: 300ms;
    border-radius: 3px;
    display: flex;
}

.item:hover {
    background: rgba(0, 0, 0, 0.05);
}

.title {
    font-weight: bold;
    display: block;
    /* padding: 0; */
    margin: 0;
}

.badge {
    width: 64px;
    display: inline-block;
    vertical-align: top;
}

.description span {
    display: block;
    color: #555555;
    padding-top: 3px;
}
</style>