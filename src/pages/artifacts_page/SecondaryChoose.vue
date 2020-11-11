<template>
    <div>
        <el-input
            v-for="(item, index) in value"
            :key="index"
            :value="item.value"
            @input="item.value = $event"
            style="margin-bottom: 8px"
        >
            <el-select
                :value="item.tag"
                @input="item.tag = $event"
                slot="prepend"
                style="width: 200px"
            >
                <el-option
                    v-for="tagItem in secondaryTag"
                    :key="tagItem"
                    :label="tagToChs[tagItem]"
                    :value="tagItem"
                >
                </el-option>
            </el-select>

            <el-button slot="append" type="danger" icon="el-icon-close"
                @click="deleteItem(index)"
            >
            </el-button>
        </el-input>

        <el-button icon="el-icon-plus" circle type="primary"
            style="margin: 8px 0"
            @click="add"
        >
        </el-button>
    </div>
</template>

<script>
import { SECONDARY_TAG, toChinese as tagToChs } from "@/common/artifacts_tag";


export default {
    name: "SecondaryChoose",
    props: {
        value: {
            type: Array,
        }
    },
    data: function() {
        return {
            tagToChs: tagToChs,
            secondaryTag: SECONDARY_TAG,

            tagName: "",
            tagValue: 0,
        }
    },
    methods: {
        add: function() {
            if (this.value.length === 4) {
                this.$message({
                    message: "副词条数量不能超过4",
                    type: "warning",
                })
                return;
            }
            this.value.push({
                tag: SECONDARY_TAG[0],
                value: 0,
            });
            this.$emit("input", this.value);
        },
        deleteItem: function(index) {
            this.value.splice(index, 1);
        }
    }
}
</script>