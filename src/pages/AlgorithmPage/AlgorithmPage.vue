<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>算法说明</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <h2 class="title">期望函数</h2>
        <div>
            <p>物理：(1 + 伤害加成 + 平A伤害加成 + 物理伤害加成) * (1 + 暴击率 * 暴击伤害) * 攻击力<br>
            元素：(1 + 伤害加成 + 元素伤害加成) * (1 + 暴击率 * 暴击伤害) * 攻击力</p>
            <p>说明：物理伤害默认使用平A打出，特殊输出手段的角色有特定目标函数</p>
        </div>

        <h2 class="title">最大输出函数</h2>
        <div>
            <p>物理：(1 + 伤害加成 + 平A伤害加成 + 物理伤害加成) * (1 + 暴击伤害) * 攻击力<br>
            元素：(1 + 伤害加成 + 元素伤害加成) * (1 + 暴击伤害) * 攻击力</p>
            <p>说明：物理伤害默认使用平A打出，特殊输出手段的角色有特定目标函数</p>
        </div>

        <h2 class="title">基于频率的输出期望函数</h2>
        <div>
            <p>
                af := 平A频率<br>
                bf := 重击频率<br>
                ef := E频率<br>
                qf := Q频率<br>
                <br>
                aT := 平A倍率<br>
                bT := 重击倍率<br>
                eT := E倍率<br>
                qT := Q倍率<br>
                <br>
                aC := 平A暴击率<br>
                bC := 重击暴击率<br>
                eC := E暴击率<br>
                qC := Q暴击率<br>
                <br>
                aBonus := 平A伤害加成<br>
                bBonus := 重击伤害加成<br>
                eBonus := E伤害加成<br>
                qBonus := Q伤害加成<br>
                bonus:= 伤害加成<br>
                eleBonus := 元素伤害加成<br>
                pBonus := 物理伤害加成<br>
                <br>
                ar := 平A元素占比<br>
                br := 重击元素占比<br>
                <br>
                c := 充能效率<br>
                cd := 暴击伤害<br>
                atk := 攻击力<br>
                <br>
                af' := af / (af + bf + ef + qf * (1 + c))<br>
                bf' := bf / (af + bf + ef + qf * (1 + c))<br>
                ef' := ef / (af + bf + ef + qf * (1 + c))<br>
                qf' := qf * (1 + c) / (af + bf + ef + qf * (1 + c))<br>
                <br>
                a := (pBonus * (1 - ar) + eleBonus * ar + aBonus + bonus + 1) * aT * (1 + aC * cd)<br>
                b := (pBonus * (1 - br) + eleBonus * br + bBonus + bonus + 1) * bT * (1 + bC * cd)<br>
                e := (1 + eleBonus + eBonus + bonus) * eT<br>
                q := (1 + eleBonus + qBonus + bonus) * qT<br>
                <br>
                <span class="bold">expect := (a * af' + b * bf' + e * ef' + q * qf') * atk</span>
            </p>
            <p>说明：刻晴、迪卢克使用了该函数</p>
        </div>
    </div>
</template>

<script>
export default {
    name: "AlgorithmPage"
}
</script>

<style scoped>
.bold {
    /* font-weight: bold; */
    background: #123456cc;
    color: white;
    border-radius: 3px;
    padding: 0 4px;
}
</style>