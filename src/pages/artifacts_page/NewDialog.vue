<template>
    <el-dialog
        title="添加圣遗物"
        :visible.sync="visible"
        width="80%"
        :before-close="onCancel"
    >

        <h2>套装 已选择【{{ toChinese[setName] }}】</h2>
        <set-choose :value="setName" @input="changeSet"></set-choose>

        <h2>位置 已选择【{{ detailName[setName][position] }}】</h2>
        <position-choose
            :value="position"
            @input="changePosition"
            :setName="setName"
        ></position-choose>        

        <h2>主属性</h2>
        <el-input v-model="primaryTagValue">
            <el-select
                v-model="primaryTagName"
                slot="prepend"
                style="width: 200px"
            >
                <el-option
                    v-for="item in primaryTag[position]"
                    :key="item"
                    :label="tagToChs[item]"
                    :value="item"
                >
                </el-option>
            </el-select>
        </el-input>

        <h2>副属性</h2>
        <secondary-choose v-model="secondaryTags"></secondary-choose>


        <h2>摘要</h2>
        <el-alert :closable="false"
            class="summary"
            :title="setName ? `套装「${toChinese[setName]}」` : '未选择套装'"
            :type="setName ? 'success' : 'error'"
        ></el-alert>
        <el-alert
            class="summary"
            :closable="false"
            :title="position ? `位置「${detailName[setName][position]}」` : '未选择位置'"
            :type="position ? 'success' : 'error'"
        >
        </el-alert>
        <el-alert
            class="summary"
            :closable="false"
            :title="checkPrimaryTag ? primaryTagSummary : '未设置主属性'"
            :type="checkPrimaryTag ? 'success' : 'error'"
        >
        </el-alert>
        <el-alert
            class="summary"
            :closable="false"
            :title="checkSecondaryTag ? '副属性设置正确' : '副属性设置错误'"
            :type="checkSecondaryTag ? 'success' : 'error'"
        >
        </el-alert>

        <template #footer>
            <el-button @click="onCancel">取消</el-button>
            <el-button type="primary" :disabled="!isOk" @click="onConfirm">确定</el-button>
        </template>
    </el-dialog>
</template>

<script>
import { SET_NAME, toChinese, DETAIL_NAME } from "@/common/artifacts_set";
import { PRIMARY_TAG, SECONDARY_TAG, toChinese as tagToChs } from "@/common/artifacts_tag";
import SetChoose from "./SetChoose";
import PositionChoose from "./PositionChoose";
import SecondaryChoose from "./SecondaryChoose";

import { deepCopy } from "@/utils/common";

export default {
    name: "NewDialog",
    components: {
        SetChoose,
        PositionChoose,
        SecondaryChoose,
    },
    props: {
        visible: {
            type: Boolean
        }
    },
    data: function() {
        return {
            // 套装名
            setName: SET_NAME[0],
            // 位置名
            position: "flower",
            // 主属性名
            primaryTagName: PRIMARY_TAG["flower"][0],
            primaryTagValue: 0,
            // 副属性
            secondaryTags: [],

            detailName: DETAIL_NAME,
            toChinese: toChinese,

            primaryTag: PRIMARY_TAG,
            secondaryTag: SECONDARY_TAG,
            tagToChs: tagToChs,
        }
    },
    methods: {
        deepCopy,
        changeSet: function(name) {
            if (!DETAIL_NAME[name][this.position]) {
                this.position = Object.keys(DETAIL_NAME[name])[0];
            }
            this.setName = name;
        },
        changePosition: function(name) {
            this.position = name;
            if (PRIMARY_TAG[name].indexOf(this.primaryTagName) === -1) {
                this.primaryTagName = PRIMARY_TAG[name][0];
            }
        },
        compact: function() { 
            return {
                setName: this.setName,
                position: this.position,
                detailName: DETAIL_NAME[this.setName][this.position],
                primary: {
                    tag: this.primaryTagName,
                    value: this.primaryTagValue,
                },
                secondary: deepCopy(this.secondaryTags),
                omit: false,
            }
        },
        onConfirm: function() {
            let result = this.compact();
            this.$emit("confirm", result);
        },
        onCancel: function() {
            this.$emit("close");
        }
    },
    computed: {
        checkPrimaryTag: function() {
            switch (this.primaryTagName) {
                // case "cureEffect":
                // case "life2":
                // case "attack2":
                // case "defend2":
                // case "critical":
                // case "criticalDamage":
                // case "recharge":
                // case ""    
                case "life1":
                case "attack1":
                case "defend1":
                case "elementalMastery":
                    return this.primaryTagValue > 0;
                default:
                    return this.primaryTagValue > 0;
            }
        },
        primaryTagSummary: function() {
            return tagToChs[this.primaryTagName] + ": " + this.primaryTagValue;
        },
        checkSecondaryTag: function() {
            if (this.secondaryTags.length === 0) {
                return false;
            }

            for (let i = 0; i < this.secondaryTags.length; i++) {
                let tag = this.secondaryTags[i];
                if (!(tag.tag && tag.value > 0)) {
                    return false;
                }
            }

            return true;
        },
        isOk: function() {
            return this.setName && this.position && this.checkPrimaryTag && this.checkSecondaryTag;
        }
    }
}
</script>

<style scoped>
.summary {
    margin-bottom: 8px;
}
</style>