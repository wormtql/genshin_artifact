<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>算法说明</el-breadcrumb-item>
            <el-breadcrumb-item>潜力函数</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row type="flex" justify="center">
            <el-col :xs="24" :sm="18">
                <h1>影响因素</h1>
                <p>众所周知，输出向副词条的影响基本可以用以下这个公式概括：</p>
                <p v-katex="'atk*(1+crit*cd)'"></p>
                <p v-katex="'atk=baseAtk*(1+p)+s'"></p>
                <p>其中，<span v-katex="'baseAtk'"></span>为基础攻击力，<span v-katex="'p'"></span>为百分比攻击力加成，<span v-katex="'s'"></span>为固定攻击力加成</p>

                <h1>期望</h1>
                <p>对于满级圣遗物<span v-katex="'X'"></span>，定义：</p>
                <p v-katex="'V=f(X)'"></p>
                <p>
                    其中，<span v-katex="'f'"></span>为潜力函数，它是圣遗物副词条的函数。<br>
                    而对于非满级圣遗物，<span v-katex="'X'"></span>是随机变量，因此：
                </p>
                <p v-katex="'V=E(f(X))'"></p>
                <p>因此，这个模型不仅可以给满级圣遗物评分，也可以给未满级圣遗物计算“潜力值”</p>

                <h1>算法</h1>
                <p>知道了上述原理，其实就是简单地进行搜索，每个子树相当于一个强化中的状态，每个叶节点就是强化到满级的状态</p>
            </el-col>
        </el-row>
    </div>
</template>

<script>
export default {
    name: "PotentialFuncPage",
    // mounted: function () {
    //     let ele = document.getElementById("text");
    //     katex.render("c = \\pm\\sqrt{a^2 + b^2}", ele);
    // }
}
</script>