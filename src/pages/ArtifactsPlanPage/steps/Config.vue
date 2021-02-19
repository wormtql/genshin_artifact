<template>
    <div>
        <h3 class="title">圣遗物套装（计算结果将限定在套装之内）</h3>
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
                <el-select
                    :disabled="artifactMode !== '2'"
                    v-model="setName1"
                >
                    <el-option
                        v-for="item in allArtifactsName"
                        :key="item.name"
                        :label="item.chs"
                        :value="item.name"
                    ></el-option>
                </el-select>
            </div>
            <div class="row">
                <el-radio v-model="artifactMode" label="22" class="radio">2+2</el-radio>
                <el-select
                    :disabled="artifactMode !== '22'"
                    v-model="setName2"
                >
                    <el-option
                        v-for="item in allArtifactsName"
                        :key="item.name"
                        :label="item.chs"
                        :value="item.name"
                    ></el-option>
                </el-select>
                <span class="plus">+</span>
                <el-select
                    :disabled="artifactMode !== '22'"
                    v-model="setName3"
                >
                    <el-option
                        v-for="item in allArtifactsName"
                        :key="item.name"
                        :label="item.chs"
                        :value="item.name"
                    ></el-option>
                </el-select>
            </div>
            <div class="row">
                <el-radio v-model="artifactMode" label="4" class="radio">4</el-radio>
                <el-select
                    :disabled="artifactMode !== '4'"
                    v-model="setName4"
                >
                    <el-option
                        v-for="item in allArtifactsName"
                        :key="item.name"
                        :label="item.chs"
                        :value="item.name"
                    ></el-option>
                </el-select>
            </div>
        </div>
        <el-button type="primary" class="confirm-button" @click="handleConfirm">确定</el-button>
    </div>
</template>

<script>
import { artifactsData } from "@asset/artifacts";

let allArtifactsName = Object.values(artifactsData).map(item => {
    return {
        name: item.eng,
        chs: item.chs,
    };
});

export default {
    name: "Config",
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