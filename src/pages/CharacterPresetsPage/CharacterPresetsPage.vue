<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>预设</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <div class="body">
            <template v-if="!isEmpty">
                <preset-item
                    v-for="(item, name) in all"
                    :key="name"
                    :item="item"
                    @delete="deletePreset(name)"
                    class="item"
                ></preset-item>
            </template>
            <el-alert
                v-else
                title="请去Artifacts Planner页面添加预设"
                :closable="false"
            ></el-alert>
        </div>
    </div>
</template>

<script>
import { mapGetters } from "vuex";

import PresetItem from "@c/display/PresetItem";

export default {
    name: "CharacterPresetsPage",
    components: {
        PresetItem,
    },
    methods: {
        deletePreset(name) {
            this.$store.commit("presets/delete", {
                name,
            });
        }
    },
    computed: {
        ...mapGetters("presets", ["all"]),
        isEmpty() {
            return Object.keys(this.all).length === 0;
        }
    }
}
</script>

<style scoped>
.item {
    margin: 0 16px 16px 0;
}

.body {
    display: flex;
    flex-wrap: wrap;
}
</style>