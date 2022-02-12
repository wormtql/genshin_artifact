<template>
    <div class="buff-root">
        <div class="top" :class="{ lock: this.lock }">
            <p class="buff-title">{{ buffCommon.chs }}</p>
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
        
        <item-config
            v-if="buffCommon.configConfig.length > 0"
            :value="buffConfig"
            @input="handleChangeConfig"
            :item-name="buff.name"
            :configs="buffCommon.configConfig"
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
        buffCommon() {
            let data = buffData[this.buff.name]

            return {
                chs: data.chs,
                configConfig: data.config,
            }
        },

        lock() {
            return this.buff.lock
        },
    }
}
</script>

<style lang="scss" scoped>
.buff-root {
    background-color: rgb(239, 246, 253);
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
        }

        .buttons {
            .button {
                padding: 0;
            }
        }
    }
}
</style>