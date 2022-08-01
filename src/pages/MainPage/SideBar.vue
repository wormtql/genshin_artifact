<template>
    <div class="root">
        <el-menu
            default-active="intro"
            style="border: none"
            @select="handleSelect"
            :mode="mode"
        >
            <el-menu-item index="/intro">
                <el-icon><i-ep-home-filled></i-ep-home-filled></el-icon>
                <span>首页</span>
            </el-menu-item>

            <el-menu-item-group>
                <template #title>
                    我的仓库
                </template>
                <el-menu-item index="/artifacts">
                    <el-icon><i-ep-help-filled /></el-icon>
                    圣遗物
                </el-menu-item>
                <el-menu-item index="/artifacts-kumi">
                    <el-icon><i-ep-help-filled /></el-icon>
                    圣遗物套装
                </el-menu-item>
                <el-menu-item index="/presets">
                    <el-icon><i-ep-menu /></el-icon>
                    计算预设
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>
                    计算
                </template>
                <el-menu-item index="/calculate">
                    <el-icon><i-ep-cpu /></el-icon>
                    计算器
                </el-menu-item>
                <el-menu-item index="/sequential-optimization">
                    <el-icon><i-ep-cpu /></el-icon>
                    队列优化
                </el-menu-item>
                <el-menu-item index="/team-optimization">
                    <el-icon><i-ep-cpu /></el-icon>
                    多人优化
                </el-menu-item>
                <el-menu-item index="/potential">
                    <el-icon><i-ep-opportunity /></el-icon>
                    圣遗物潜力
                </el-menu-item>
                <el-menu-item index="/character">
                    <el-icon><i-ep-histogram /></el-icon>
                    莫娜数据库
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>附加功能</template>
                <el-menu-item index="/playground">
                    <el-icon><i-fa6-solid-terminal /></el-icon>
                    Playground
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>
                    关于本站
                </template>
                <el-sub-menu index="help">
                    <template #title>
                        <el-icon><i-ep-question-filled /></el-icon>
                        <i class="el-icon-question"></i>
                        帮助
                    </template>
                    <el-menu-item index="/help/export-tools">
                        <el-icon><i-ep-aim /></el-icon>
                        导出工具
                    </el-menu-item>
                </el-sub-menu>

                <el-menu-item index="/tomodachi">
                    <el-icon><i-ep-link /></el-icon>
                    友情链接
                </el-menu-item>
            </el-menu-item-group>
        </el-menu>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue"

export default defineComponent({
    name: "SideBar",
    props: {
        items: {
            type: Array,
            default: () => [],
        },
        mode: {
            type: String,
            default: "vertical",
        },
        doRoute: {
            type: Boolean,
            default: true,
        },
        enableMinimize: {
            type: Boolean,
            default: false
        }
    },
    data() {
        return {
            minimized: false,
        }
    },
    methods: {
        handleSelect(index: string) {
            if (this.doRoute) {
                this.$router.push(index)
            } else {
                this.$emit("select", index)
            }
        }
    }
})
</script>

<style scoped>
.top {
    height: 220px;
    background: #409eff;
    display: flex;
    align-items: center;
    justify-content: center;
}

.item {
    padding: 0 32px;
}
</style>
