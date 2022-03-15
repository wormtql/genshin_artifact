<template>
    <div class="buff-root">
        <div class="top" :class="{ lock: this.lock }">
            <p class="buff-title">{{ data.chs }}</p>
            <div class="buttons">
                <el-button
                    type="text"
                    class="button"
                    icon="el-icon-delete"
                    size="mini"
                    @click="$emit('delete')"
                ></el-button>
                <el-button
                    type="text"
                    class="button"
                    :icon="lock ? 'el-icon-unlock' : 'el-icon-lock'"
                    size="mini"
                    @click="$emit('toggle')"
                ></el-button>
            </div>
        </div>

        <div class="detail">
            <img :src="data.badge">
            <div>
                <p v-if="data.description"><span v-html="data.description"></span></p>
            </div>
        </div>
        
        <item-config
            v-if="data.config.length > 0"
            :value="buffConfig"
            @input="handleChangeConfig"
            :item-name="buff.name"
            :configs="data.config"
        ></item-config>
    </div>
</template>

<script>
import { buffData } from "@buff"

import ItemConfig from "@c/config/ItemConfig"

export default {
    name: "BuffItem",
    components: { ItemConfig },
    props: ["buff", "buffConfig"],
    methods: {
        handleChangeConfig(v) {
            this.$emit("update:buffConfig", v)
        }
    },
    computed: {
        data() {
            return buffData[this.buff.name]
        },

        lock() {
            return this.buff.lock
        },
    }
}
</script>

<style lang="scss" scoped>
.buff-root {
    //background-color: rgb(239, 246, 253);
    margin-bottom: 12px;

    .top {
        border-bottom: 1px solid #DCDFE6;
        padding: 6px 12px;
        display: flex;
        justify-content: space-between;
        align-items: center;
        transition-duration: 200ms;

        &.lock {
            // background-image: linear-gradient(45deg, #ff5272 25%, #ffffff 25%, #ffffff 50%, #ff5272 50%, #ff5272 75%, #ffffff 75%, #ffffff 100%);
            background: linear-gradient(45deg, #ffd4e2 25%, #fff 25%, #fff 50%, #ffd4e2 50%, #ffd4e2 75%, #fff 75%, #fff 100%);
            background-size: 40px 40px;
        }

        .buff-title {
            font-weight: bold;
            font-size: 14px;
            margin: 0px;
            color: #666666;
            margin-right: 24px;
        }

        .buttons {
            display: flex;
            .button {
                padding: 0;
            }
        }
    }

    .detail {
        display: flex;
        padding: 8px;

        img {
            height: 64px;
            width: 64px;
            border-radius: 50%;
            margin-right: 12px;
        }

        p {
            margin: 0;
            font-size: 12px;
            color: #606266;
        }
    }
}
</style>