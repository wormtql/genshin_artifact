<template>
    <div>
        <el-alert
            title="警告：添加过多有效词条会使得计算缓慢（指数级变慢），因此，请慎重考虑哪些是有效词条，建议数量不超过6条"
            :closable="false"
            type="warning"
            style="margin-bottom: 16px"
        ></el-alert>

        <div class="config-item">
            <h3 class="config-title">有效词条</h3>
            <el-checkbox-group v-model="validTag">
                <el-checkbox-button
                    v-for="tag in tags"
                    :key="tag.name"
                    :label="tag.name"
                >{{ tag.chs }}</el-checkbox-button>
            </el-checkbox-group>
        </div>
    </div>
</template>

<script>
import { artifactsSecondaryTag } from "@asset/artifacts";
import { secondaryTags } from "@asset/tags";

export default {
    name: "Ratio.ptcfg",
    created() {
        let temp = {};
        for (let i of artifactsSecondaryTag) {
            temp[i] = secondaryTags[i];
        }
        console.log(temp);

        this.tags = temp;
    },
    data() {
        return {
            validTag: ["attackPercentage", "critical", "criticalDamage"],
        }
    }
}
</script>