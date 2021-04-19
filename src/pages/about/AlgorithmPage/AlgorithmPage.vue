<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>算法说明</el-breadcrumb-item>
            <el-breadcrumb-item>目标函数</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row type="flex" justify="center">
            <el-col :xs="24" :sm="18">
                <h1>期望函数</h1>
                <p v-katex="'V=atk*(1+crit*cd)*(1+bonus)'"></p>

                <h1>最大输出函数</h1>
                <p v-katex="'V=atk*(1+cd)*(1+bonus)'"></p>

                <h1>基于频率的输出期望函数</h1>
                
                <p v-katex="'V=atk*(a*f_a\'+b*f_b\'+e*f_e\'+q*f_q\')'"></p>
                <p v-katex="'f_a\'=\\frac{f_a}{f_a+f_b+f_e+f_q*(1+c)}'"></p>
                <p v-katex="'f_b\'=\\frac{f_b}{f_a+f_b+f_e+f_q*(1+c)}'"></p>
                <p v-katex="'f_e\'=\\frac{f_e}{f_a+f_b+f_e+f_q*(1+c)}'"></p>
                <p v-katex="'f_q\'=\\frac{f_q*(1+c)}{f_a+f_b+f_e+f_q*(1+c)}'"></p>
                <p v-katex="'a=(1+bonus_p*(1-r_a)+bonus_{ele}*r_a+bonus_a+bonus_{other})*T_a*(1+cd*c_a)'"></p>
                <p v-katex="'b=(1+bonus_p*(1-r_b)+bonus_{ele}*r_b+bonus_b+bonus_{other})*T_b*(1+cd*c_b)'"></p>
                <p v-katex="'e=(1+bonus_{ele}+bonus_e+bonus_{other})*T_e*(1+cd*c_e)'"></p>
                <p v-katex="'q=(1+bonus_{ele}+bonus_q+bonus_{other})*T_q*(1+cd*c_q)'"></p>
                <p>
                    其中，<br>
                    <span v-katex="'f_x'"></span>为技能x的频率，<span v-katex="'f_x\''"></span>为算上充能效率后的等效频率，<br>
                    <span v-katex="'T_x'"></span>为技能x的倍率<br>
                    <span v-katex="'c_x'"></span>为技能x的暴击率<br>
                    <span v-katex="'r_x'"></span>为技能x的附魔占比<br>
                    <span v-katex="'bonus_p'"></span>为物理伤害加成，<span v-katex="'bonus_{ele}'"></span>为元素伤害加成<br>
                    <span v-katex="'c'"></span>为充能效率，<span v-katex="'cd'"></span>为暴击伤害
                </p>
                <p>说明：刻晴、迪卢克使用了该函数</p>
            </el-col>
        </el-row>
    </div>
</template>

<script>
export default {
    name: "AlgorithmPage"
}
</script>