<template>
    <div>
        <el-alert
            title="如果使用模型初始分值的话，建议是可以保留3到3.1分及以上的圣遗物胚子，这个得分是初始就有两段优质副词条且主属性合适的C位圣遗物，或者是三个优质副词条的辅助圣遗物。"
            :closable="false"
            type="warning"
            class="hint"
        ></el-alert>
        <el-alert
            title="已知缺陷：缺乏对于主属性的判断，如极品双爆大攻击胚子会赋予攻击C最高评分(即便是生命主属性情况下)"
            :closable="false"
            type="warning"
            class="hint"
        ></el-alert>
        <div class="config-item">
            <h3 class="config-title">评分机制应用场合</h3>
            <p style="color: #909399; font-size: 13px">
                本评分机制主要应用于：在圣遗物0级时判断圣遗物潜力，从而为后续可能抽到(或新出)的角色保留一定数量且质量较优的圣遗物胚子。
                举例：当前我并没有阿贝多且我未来不确定是否会抽阿贝多(也有可能抽其它以防御为核心属性的角色)，但手上有一批防御沙漏，在词条并不极品(无双爆)的情况下，我仍然想要留下一定数量未来能够升级使用的“基础圣遗物(参考上述刷取阶段)”胚子，避免抽出(或歪出)后只能使用副词条质量较差的圣遗物，并避免极品胚子强化暴毙。
                适合人群：
                <ui>
                    <li>纠结症患者，在无法量化对比的几件圣遗物之间摇摆不定；</li>
                    <li>已经有或者是习惯于刷“比较多”的胚子再统一处理的同学，需要一个标准帮助快速进行过滤；</li>
                    <li>对于未来可能出现的各种角色都有一定期待，想锁住胚子到时候再看是否拉起；</li>
                </ui>
            </p>
            <p style="margin: 0">
                总结：没资源或者没心思把每件圣遗物都升满20级再看的旅行者
            </p>
        </div>
        <div class="config-item">
            <h3 class="config-title">NGA 原帖地址</h3>
            <a href="https://ngabbs.com/read.php?tid=26432895" target="_blank" class="el-button no-deco">全角色全玩法圣遗物胚子评分+角色定位一览</a>
        </div>
        <div class="config-item">
            <h3 class="config-title">模型赋分公式</h3>
            <img :src="formula" class="image">
        </div>
        <div class="config-item">
            <h3 class="config-title" style="margin-bottom: 0">算法作者: @Meacca路亚</h3>
        </div>
    </div>

</template>

<script>
import formula from "@asset/images/initial-formula.jpeg";

export default {
    name: 'InitialPotentialConfig',
    created: function () {
        this.formula = formula
    },
    data: function() {
        return {
            initialMode: true,
        };
    },
};
</script>

<style scoped>
.hint {
    margin-bottom: 16px;
}
</style>
