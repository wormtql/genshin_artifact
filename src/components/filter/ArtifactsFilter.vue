<template>
    <div>
        <div class="item">
            <span class="title">套装：</span>
            <select-artifact-set
                :value="setName"
                @input="updateSetName"
                any-option
            ></select-artifact-set>
        </div>
        <div class="item">
            <span class="title">等级（>=）：</span>
            <el-input-number
                :min="0"
                :max="20"
                :value="minLevel"
                @input="updateMinLevel"
                size="small"
            ></el-input-number>
        </div>
    </div>
</template>

<script>
import SelectArtifactSet from "@c/select/SelectArtifactSet";

export default {
    name: "ArtifactsFilter",
    components: {
        SelectArtifactSet,
    },
    props: ["filter"],
    data() {
        return {
            setName: "any",
            minLevel: 0,
        }
    },
    methods: {
        doUpdate() {
            this.$emit("update:filter", this.fil);
        },

        updateSetName(name) {
            this.setName = name;
            this.doUpdate();
        },

        updateMinLevel(level) {
            this.minLevel = level;
            this.doUpdate();
        }
    },
    computed: {
        filterSetName() {
            return (item) => {
                if (this.setName === "any") {
                    return true;
                }
                return item.setName === this.setName;
            };
        },

        filterMinLevel() {
            return item => {
                return (item.level ?? 20) >= this.minLevel;
            }
        },

        fil() {
            return item => {
                return this.filterSetName(item) && this.filterMinLevel(item);
            }
        }
    }
}
</script>

<style lang="scss" scoped>
.item {
    display: inline-block;
    margin-right: 16px;

    .title {
        color: #606266;
        font-size: 14px;
    }
}
</style>