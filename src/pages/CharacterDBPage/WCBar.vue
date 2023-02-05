<template>
    <div class="wc-bar">
        <div class="header-item">
            <el-tooltip :content="tooltip">
                <img :src="imageUrl" class="header-image">
            </el-tooltip>
        </div>

        <div class="bar" :style="barStyle">{{ (item[1] * 100).toFixed(1) }}%</div>
    </div>
</template>

<script>
import { weaponData } from "@weapon"
import {useI18n} from "../../i18n/i18n";

export default {
    name: "WCBar",
    props: ["item", "type"],
    computed: {
        imageUrl() {
            if (this.type === "weapon") {
                return weaponData[this.item[0]].url
            }
        },

        tooltip() {
            const { ta } = useI18n()
            if (this.type === "weapon") {
                const nameLocaleIndex = weaponData[this.item[0]].nameLocale
                return ta(nameLocaleIndex)
            }
        },

        barStyle() {
            return {
                width: `calc((100% - 96px - 12px) * ${this.item[1]})`
            }
        }
    }
}
</script>

<style scoped lang="scss">
.wc-bar {
    display: flex;
    align-items: center;
}

.header-item {
    width: 96px;
    margin-right: 12px;

    .header-image {
        width: 48px;
        height: 48px;
        border-radius: 50%;
    }
}

$bar-height: 32px;
.bar {
    height: $bar-height;
    background-color: #409EFF;
    //border-radius: 3px;
    font-size: 12px;
    color: #606166;
    line-height: $bar-height;
    padding-left: 12px;
    box-sizing: border-box;
}
</style>