<template>
    <div>
        <el-input
            v-for="index in 4"
            :key="index"
            :value="value[index - 1].value"
            @input="handleValueChange(index - 1, $event)"
            size="small"
            class="input"
        >
            <template slot="prepend">
                <el-select
                    :value="value[index - 1].name"
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
    </div>
</template>

<script>
import { artifactsSecondaryTag } from "../../assets/artifacts";
import { secondaryTags } from "../../assets/tags";

let secondaryTagData = artifactsSecondaryTag.map(item => secondaryTags[item]);

export default {
    name: "ChooseNormalTag",
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
            this.value[index].value = value;
            this.$emit("input", this.value);
        },

        handleTagChange(index, value) {
            this.value[index].name = value;
            this.$emit("input", this.value);
        }
    },
    computed: {
        isPercentage() {
            let temp = [];

            for (let i = 0; i < 4; i++) {
                try {
                    temp.push(secondaryTags[this.value[i].name].percentage);
                } catch(e) {
                    temp.push(false);
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

.input {
    margin-bottom: 12px;
}
</style>