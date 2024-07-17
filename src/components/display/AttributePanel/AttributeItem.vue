<template>
    <div class="attribute-item">
        <el-tooltip>
            <span class="title">{{ title }}</span>
            <template #content>
<!--                <div slot="content">-->
                    <attribute-composition :composition="composition"></attribute-composition>
<!--                </div>-->
            </template>
        </el-tooltip>
        <span class="value">{{ displayValue }}</span>
    </div>
</template>

<script>
import AttributeComposition from "./AttributeComposition"

export default {
    name: "AttributeItem",
    components: {
        AttributeComposition
    },
    props: {
        composition: {},
        title: {},
        percentage: {
            default: false
        }
    },
    computed: {
        sum() {
            let temp = 0
            for (const key in this.composition) {
                temp += this.composition[key]
            }
            return temp
        },

        displayValue() {
            if (this.percentage) {
                return `${(this.sum * 100).toFixed(1)}%`
            } else {
                return Math.round(this.sum)
            }
        }
    }
}
</script>

<style lang="scss" scoped>
.attribute-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    cursor: default;
    font-size: 14px;
    padding: 4px;

    .title {
        color: #909399;
        font-weight: bold;
    }
}
</style>
