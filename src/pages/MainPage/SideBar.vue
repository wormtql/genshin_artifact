<template>
    <div class="root">
        <el-menu
            default-active="intro"
            style="border: none"
            @select="handleSelect"
            :mode="mode"
        >
            <el-menu-item index="/intro">
                <i class="el-icon-s-home"></i>
                <span>首页</span>
            </el-menu-item>

            <el-menu-item-group>
                <template #title>
                    我的仓库
                </template>
                <el-menu-item index="/artifacts">
                    <i class="el-icon-s-help"></i>
                    圣遗物
                </el-menu-item>
                <el-menu-item index="/artifacts-kumi">
                    <i class="el-icon-s-help"></i>
                    圣遗物套装
                </el-menu-item>
                <el-menu-item index="/presets">
                    <i class="el-icon-menu"></i>
                    计算预设
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>
                    计算
                </template>
                <el-menu-item index="/calculate">
                    <i class="el-icon-cpu"></i>
                    计算器
                </el-menu-item>
                <el-menu-item index="/team-optimization">
                    <i class="el-icon-s-opportunity"></i>
                    多人优化
                </el-menu-item>
                <el-menu-item index="/potential">
                    <i class="el-icon-magic-stick"></i>
                    圣遗物潜力
                </el-menu-item>
                <el-menu-item index="/character">
                    <i class="el-icon-s-data"></i>
                    莫娜数据库
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>附加功能</template>
                <el-menu-item index="/playground">
                    <font-awesome-icon :icon="['fas', 'terminal']" style="width: 24px; margin-right: 5px; color: #909399"></font-awesome-icon>
                    Playground
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>
                    关于本站
                </template>
                <el-submenu index="help">
                    <template slot="title">
                        <i class="el-icon-question"></i>
                        帮助
                    </template>
                    <el-menu-item index="/help/export-tools">
                        <i class="el-icon-aim"></i>
                        导出工具
                    </el-menu-item>
                </el-submenu>

                <el-menu-item index="/tomodachi">
                    <i class="el-icon-link"></i>
                    友情链接
                </el-menu-item>
            </el-menu-item-group>
        </el-menu>
    </div>
</template>

<script>
export default {
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
        handleSelect(index) {
            if (this.doRoute) {
                this.$router.push(index)
            } else {
                this.$emit("select", index)
            }
        }
    }
}
</script>

<style scoped>
.top {
    height: 220px;
    background: #409eff;
    display: flex;
    align-items: center;
    justify-content: center;
}

/* .root {
    background: #123456;
} */

.item {
    padding: 0 32px;
}
</style>