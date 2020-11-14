<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>使用示例</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <h3>1. 导入圣遗物</h3>
        <p>点击左侧“添加圣遗物”，进入页面后点击左上角蓝色按钮“添加圣遗物”，输入属性之后点击确定</p>
        <p>重复上述步骤，直到将所有待选圣遗物都导入</p>
        <p>导入圣遗物后可以点击右上角“导出json”按钮，保存导出的字符串，下次可以直接点击“导入json”按钮，不用再重新输入</p>
        <p>完成后如下图：</p>
        <el-image :src="publicPath + 'help/arts.png'" lazy></el-image>

        <h3>2. 选择角色和武器</h3>
        <p>点击左侧“Arts Planner”，进入页面，点击“角色”，选中想要的角色，点击“武器”，选中想要的武器</p>
        <el-image :src="publicPath + 'help/select-character.png'" lazy></el-image>
        <el-image :src="publicPath + 'help/select-weapon.png'" lazy></el-image>

        <h3>3. 选择目标函数</h3>
        <p>点击“目标函数”按钮。下面的选项卡有多种目标函数</p>
        <p>“角色预设”是为角色专门准备的培养方案，切记只在选中对应角色时才选角色预设。由于之前选了刻晴+黑剑，此处选择“常规刻晴”</p>
        <p>“基础属性”可以求得单个属性值的最大化或者许多与角色无关的属性，例如，如果选择“攻击力”，则可以求得使得攻击力最大化的搭配</p>
        <el-image :src="publicPath + 'help/select-target.png'" lazy></el-image>

        <h3>4. 开始计算</h3>
        <p>点击“计算”按钮，开始计算最佳搭配，计算完成后弹出对话框，部分目标函数会显示<a>梯度</a>信息，关于梯度，请参考“算法”页面</p>
        <p>关闭对话框后，会在右侧面板预览页面展示搭配该圣遗物后的最终面板</p>
        <el-image :src="publicPath + 'help/result.png'" lazy></el-image>

    </div>
</template>

<script>
export default {
    name: "UseCasePage",
    data: function() {
        return {
            publicPath: process.env.BASE_URL,
        }
    }
}
</script>