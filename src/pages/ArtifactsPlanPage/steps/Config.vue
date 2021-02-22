<template>
    <div>
        <h3 class="title">圣遗物套装（计算结果将限定在套装之内）</h3>
        <el-alert
            :closable="false"
            title="某些动态的加成不会考虑，所以对于非常适合某个角色的圣遗物，可以考虑限定套装，例如迪卢克限定魔女4，甘雨限定冰4"
            style="margin-bottom: 16px"
        ></el-alert>
        <div>
            <div class="row">
                <el-radio
                    v-model="artifactMode"
                    label="any"
                    class="radio"
                >任意</el-radio>
            </div>
            <div class="row">
                <el-radio v-model="artifactMode" label="2" class="radio">2</el-radio>
                <select-artifact-set v-model="setName1" :disabled="artifactMode !== '2'"></select-artifact-set>
            </div>
            <div class="row">
                <el-radio v-model="artifactMode" label="22" class="radio">2+2</el-radio>
                <select-artifact-set v-model="setName2" :disabled="artifactMode !== '22'"></select-artifact-set>
                <span class="plus">+</span>
                <select-artifact-set v-model="setName3" :disabled="artifactMode !== '22'"></select-artifact-set>
            </div>
            <div class="row">
                <el-radio v-model="artifactMode" label="4" class="radio">4</el-radio>
                <select-artifact-set v-model="setName4" :disabled="artifactMode !== '4'"></select-artifact-set>
            </div>
        </div>
        <el-button type="primary" class="confirm-button" @click="handleConfirm">确定</el-button>
    </div>
</template>

<script>
import { artifactsData } from "@asset/artifacts";

import SelectArtifactSet from "@c/SelectArtifactSet";

let allArtifactsName = Object.values(artifactsData).map(item => {
    return {
        name: item.eng,
        chs: item.chs,
    };
});

export default {
    name: "Config",
    components: {
        SelectArtifactSet,
    },
    created: function () {
        this.allArtifactsName = allArtifactsName;
    },
    data: function () {
        return {
            artifactMode: "any",
            setName1: "berserker",
            setName2: "archaicPetra",
            setName3: "archaicPetra",
            setName4: "archaicPetra",
        }
    },
    methods: {
        handleConfirm() {
            let temp = {
                mode: this.artifactMode,
            }
            if (this.artifactMode === "2") {
                temp.setName1 = this.setName1;
            } else if (this.artifactMode === "22") {
                temp.setName1 = this.setName2;
                temp.setName2 = this.setName3;
            } else if (this.artifactMode === "4") {
                temp.setName1 = this.setName4;
            }

            this.$emit("select", temp);
        }
    }
}
</script>

<style scoped>
.title {
    /* background:rgb(74, 99, 211); */
    padding: 0px 16px;
    display: inline-block;
    /* color: white; */
    border-radius: 3px;
    font-size: 14px;
    border-bottom: 5px solid rgb(74, 99, 211);
    color: #555555;
}

.radio {
    width: 80px;
}

.row {
    margin-bottom: 18px;
}

.plus {
    padding: 0 8px;
    color: #606266;
}

.confirm-button {
    width: 100%;
}
</style>