<template>
    <div class="root">
        <!-- <div class="top">
            {{ "《原神》数值计算" }}
            
        </div> -->
        <el-menu
            default-active="intro"
            style="border: none"
            router
        >
            <el-menu-item index="intro">
                <i class="el-icon-s-home"></i>
                <span>首页</span>
            </el-menu-item>

            <el-menu-item-group>
                <template #title>
                    自定义
                </template>
                <el-menu-item index="custom-character">
                    <i class="el-icon-user-solid"></i>
                    <span>角色</span>
                </el-menu-item>
                <el-menu-item index="custom-weapon">
                    <i class="el-icon-scissors"></i>
                    <span>武器</span>
                </el-menu-item>
                <el-menu-item index="custom-target">
                    <i class="el-icon-s-promotion"></i>
                    <span>目标函数</span>
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>
                    圣遗物
                </template>
                <el-menu-item index="artifacts">
                    <i class="el-icon-s-help"></i>
                    添加圣遗物
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>
                    计算
                </template>
                <el-menu-item index="calculate">
                    <i class="el-icon-cpu"></i>
                    Arts Planner
                </el-menu-item>
                <el-menu-item index="panel-calculate">
                    <i class="el-icon-cpu"></i>
                    <span>面板计算</span>
                </el-menu-item>
            </el-menu-item-group>

            <el-menu-item-group>
                <template #title>
                    帮助
                </template>
                <el-menu-item index="use-case">
                    <i class="el-icon-question"></i>
                    完整使用示例
                </el-menu-item>
                <el-menu-item index="alg">
                    <i class="el-icon-s-opportunity"></i>
                    算法
                </el-menu-item>
            </el-menu-item-group>
            <!-- <el-menu-item v-for="item in items" :key=item.index :index="item.name">
                <i :class="item.icon"></i>
                <span>{{ item.text }}</span>
            </el-menu-item> -->
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

.root {
    /* background: #123456; */
}

.item {
    padding: 0 32px;
}
</style>