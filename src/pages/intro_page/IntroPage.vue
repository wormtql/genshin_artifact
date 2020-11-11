<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>主页</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <h3>介绍</h3>
        <p>计算最佳圣遗物搭配，根据不同的目标函数有不同的搭配。</p>

        <h3>使用方法</h3>
        <el-steps :active="100">
            <el-step title="写入圣遗物" icon="el-icon-s-help"></el-step>
            <el-step title="配置基础属性" description="包括角色、武器"></el-step>
            <el-step title="配置目标函数" description="不同的养成计划有不同的目标函数 "></el-step>
        </el-steps>
        
        <h3>自定义</h3>
        <p>由于数据不完善，只支持少部分等级的武器和角色，因此可以自定义</p>

        <h3>注意</h3>
        <p>由于是枚举的算法，因此数据量不能太大，添加圣遗物的时候应当把明显更次的圣遗物去掉或者禁用</p>

        <h3>项目地址</h3>
        <p>面板计算器：<a href="https://github.com/wormtql/genshin_panel">https://github.com/wormtql/genshin_panel</a></p>
        <p>前端：<a href="https://github.com/wormtql/genshin_artifact">https://github.com/wormtql/genshin_artifact</a></p>

        <h3>数据</h3>
        <p>网站使用的数据来源：</p>
        <ul>
            <li><a href="https://wiki.biligame.com/ys/%E9%A6%96%E9%A1%B5">bilibili原神wiki</a></li>
        </ul>
    </div>
</template>

<script>
export default {
    name: "IntroPage"
}
</script>