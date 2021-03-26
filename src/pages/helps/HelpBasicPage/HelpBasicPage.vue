<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>帮助</el-breadcrumb-item>
            <el-breadcrumb-item>基本使用</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <div class="body">
            <h1>1. 添加圣遗物</h1>
            <p>单击“添加圣遗物”按钮，在弹出的对话框输入圣遗物详情，可以手动添加单个圣遗物</p>
            <img src="./img1.png">

            <p>单击“导入json”按钮，可以从json批量导入圣遗物，json是一种表示数据的格式，详情请参考<a href="https://www.json.org/json-zh.html" target="_blank">json官网</a></p>
            <p>这里的圣遗物将会是整个应用的核心数据，配装、潜力计算都依赖这里的圣遗物</p>
            <p>目前已有的与本站兼容的第三方自动圣遗物导出工具：</p>
            <ul>
                <li><a target="_blank" href="https://github.com/ChanIok/JumpyDumpty">蹦蹦炸弹</a></li>
            </ul>

            <h1>2. 配装</h1>
            <p>选择“Arts Planner”面板，此处可以进行配装</p>
            <p>依次选择角色、角色参数（等级、命座等）、武器、武器参数、目标、目标参数、配置，最后单击“开始计算”，进行计算</p>
            <img src="./img2.png">


            <h1>预设</h1>
            <p>点击“存为预设”按钮，可以保存当前设置的角色、角色参数、武器、武器参数、目标、目标参数。注意：不保存“配置”的内容</p>
            <p>点击“应用预设”按钮，可以应用已经保存了的预设，以避免重新输入参数</p>
            </div>
    </div>
</template>

<script>
export default {
    name: "HelpBasicPage",
}
</script>

<style scoped>
.body {
    padding: 0 150px;
}
</style>