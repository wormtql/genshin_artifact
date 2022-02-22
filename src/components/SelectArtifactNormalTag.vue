<template>
    <div>
        <div
            v-for="index in 4"
            :key="index"
            class="item"
        >
            <el-input
                :value="displayedValue[index - 1]"
                @input="handleValueChange(index - 1, $event)"
                size="small"
                class="input"
            >
                <template slot="prepend">
                    <el-select
                        :value="names[index - 1]"
                        @input="handleTagChange(index - 1, $event)"
                        class="prepend"
                    >
                        <el-option
                            v-for="tagData in secondaryTagData"
                            :key="tagData.name"
                            :label="tagData.chs"
                            :value="tagData.name"
                        >
                        </el-option>
                    </el-select>
                </template>

                <template slot="append">
                    <span v-if="isPercentage[index - 1]">%</span>
                </template>
            </el-input>

            <el-button
                icon="el-icon-delete"
                circle
                type="text"
                class="del-button"
                @click="deleteItem(index - 1)"
            ></el-button>
        </div>
    </div>
</template>

<script>
import { artifactsSecondaryTag } from "@asset/artifacts";
import { artifactTags } from "@const/artifact";
// import { getArtifactRealValue } from "@util/utils";
import deepCopy from "@util/deepcopy";

let secondaryTagData = artifactsSecondaryTag.map(item => artifactTags[item]);

export default {
    name: "SelectArtifactNormalTag",
    created: function () {
        this.secondaryTagData = secondaryTagData;
    },
    props: {
        value: {
            type: Array,
            required: true,
        }
    },
    methods: {
        handleValueChange(index, value) {
            if (!this.value[index]) {
                return;
            }

            let temp = deepCopy(this.value);
            temp[index].value = value;

            this.$emit("input", temp);
        },

        handleTagChange(index, value) {
            let temp = deepCopy(this.value);
            if (this.value[index] && this.value[index].name !== "") {
                temp[index].name = value;
            } else {
                temp.push({
                    name: value,
                    value: 0,
                });
            }

            this.$emit("input", temp);
        },

        deleteItem(index) {
            let temp = [];
            for (let i = 0; i < this.value.length; i++) {
                if (i !== index) {
                    temp.push(deepCopy(this.value[i]));
                }
            }

            this.$emit("input", temp);
        }
    },
    computed: {
        isPercentage() {
            let temp = [];

            for (let i = 0; i < 4; i++) {
                try {
                    temp.push(artifactTags[this.value[i].name].percentage);
                } catch(e) {
                    temp.push(false);
                }
            }
            
            return temp;
        },

        displayedValue() {
            let temp = [];
            for (let i = 0; i < 4; i++) {
                try {
                    let x = this.value[i].value;
                    temp.push(x);
                } catch(e) {
                    temp.push("0");
                }
            }

            return temp;
        },

        names() {
            let temp = [];
            for (let i = 0; i < 4; i++) {
                try {
                    let x = this.value[i].name;
                    temp.push(x);
                } catch(e) {
                    temp.push("");
                }
            }

            return temp;
        }
    }
}
</script>

<style scoped>
.prepend {
    width: 150px;
}

.item {
    display: flex;
    align-items: center;
    margin-bottom: 12px;
}

.del-button {
    padding: 0;
    margin-left: 8px;
}
</style>