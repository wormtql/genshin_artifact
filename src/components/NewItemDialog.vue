<template>
    <el-dialog :visible.sync="show" :title="title" width="80%"
        :before-close="onCancel"
    >
        <!-- <el-alert
            title="注意：如果副属性为暴击率，则应该设置两条，分别为"
        >
        </el-alert> -->

        <h2>名称</h2>
        <el-input v-model="name" class="input"></el-input>
        <el-tag v-show="!checkName[0]" type="danger">{{ checkName[1] }}</el-tag>

        <h2>{{ primaryName }}</h2>
        <!-- <div><el-tag style="margin-bottom: 8px">注意百分数应写为小数，例如3%应写成0.03</el-tag></div> -->
        <el-button
            icon="el-icon-plus"
            circle
            type="primary"
            @click="addPrimaryTag"
            style="margin-bottom: 16px"
        ></el-button>

        <el-input
            v-for="(tag, index) in primaryTags"
            :key="'p' + index"
            v-model="tag.value"
            class="input"
        >
            <el-select slot="prepend" v-model="tag.tag" style="width: 200px">
                <el-option
                    v-for="item in primaryTagsList"
                    :key="item.eng"
                    :label="item.chs"
                    :value="item.eng"
                ></el-option>
            </el-select>

            <el-button slot="append" icon="el-icon-delete" @click="deletePrimary(index)"></el-button>
        </el-input>


        <h2>{{ secondaryName }}</h2>
        <el-button
            icon="el-icon-plus"
            circle
            type="primary"
            @click="addSecondaryTag"
            style="margin-bottom: 16px"
        ></el-button>
        <el-input
            v-for="(tag, index) in secondaryTags"
            :key="'s' + index"
            v-model="tag.value"
            class="input"
        >
            <el-select slot="prepend" v-model="tag.tag" style="width: 200px">
                <el-option
                    v-for="item in secondaryTagsList"
                    :key="item.eng"
                    :label="item.chs"
                    :value="item.eng"
                ></el-option>
            </el-select>

            <el-button slot="append" icon="el-icon-delete" @click="deleteSecondary(index)"></el-button>
        </el-input>

        <span slot="footer">
            <el-button @click="onCancel">取消</el-button>
            <el-button type="primary" @click="onConfirm"
                :disabled="!checkAll[0]"
            >确定</el-button>
        </span>
    </el-dialog>
</template>

<script>
import { primaryTagsList, secondaryTagsList } from "@/common/const/tags";
import { chsPrimaryTag, chsSecondaryTag } from "@/common/chs";
import { toRealValue } from "@/utils";
// import { mapState } from "vuex";

export default {
    name: "NewItemDialog",
    props: {
        show: {
            type: Boolean,
            default: false
        },
        title: {
            type: String,
        },
        usedNames: {
            type: Array,
            default: () => [],
        },
        primaryName: {
            type: String,
            default: "基础属性",
        },
        secondaryName: {
            type: String,
            default: "副属性",
        }
    },
    data: function() {
        return {
            primaryTagsList: primaryTagsList.map(item => ({
                    eng: item,
                    chs: chsPrimaryTag(item)
            })),
            secondaryTagsList: secondaryTagsList.map(item => ({
                    eng: item,
                    chs: chsSecondaryTag(item)
            })),

            primaryTags: [{tag: "attack", value: "100"}],
            secondaryTags: [{tag: "attack2", value: "20"}],
            name: "",
        }
    },
    methods: {
        onCancel() {
            this.$emit("cancel");
        },
        onConfirm() {
            this.$emit("confirm", this.compact());
        },
        addPrimaryTag() {
            this.primaryTags.push({
                tag: "attack",
                value: "0"
            })
        },
        deletePrimary(index) {
            if (this.primaryTags.length === 1) {
                return;
            }
            this.primaryTags.splice(index, 1);
        },
        addSecondaryTag() {
            this.secondaryTags.push({
                tag: "attack2",
                value: 0,
            });
        },
        deleteSecondary(index) {
            if (this.secondaryTags.length === 1) {
                return;
            }
            this.secondaryTags.splice(index, 1);
        },
        compact() {
            let primary = {};
            for (let i = 0; i < this.primaryTags.length; i++) {
                let name = this.primaryTags[i].tag;
                let value = toRealValue(name, Number(this.primaryTags[i].value));
                if (primary[name]) {
                    primary[name] += value;
                } else {
                    primary[name] = value;
                }
            }

            let secondary = {};
            for (let i = 0; i < this.secondaryTags.length; i++) {
                let name = this.secondaryTags[i].tag;
                let value = toRealValue(name, Number(this.secondaryTags[i].value));
                if (secondary[name]) {
                    secondary[name] += value;
                } else {
                    secondary[name] = value;
                }
            }

            return {
                item: {
                    primary, secondary
                },
                name: this.name
            }
        }
    },
    computed: {
        checkName() {
            if (this.name === "") {
                return [false, "不能为空"];
            }
            // window.console.log(this.usedNames);
            if (this.usedNames.indexOf(this.name) !== -1) {
                return [false, "名称已经存在"];
            }

            return [true];
        },
        checkAll() {
            let cn = this.checkName;
            if (!cn[0]) {
                return cn;
            }

            return [true];
        }
    }
}
</script>

<style scoped>
.input {
    margin-bottom: 8px;
}
</style>