<template>
    <el-tabs type="border-card">
        <el-tab-pane label="生之花">
            <select-single-artifact
                :data="flower"
                :value="value[0]"
                @input="changeFlower"
            ></select-single-artifact>
        </el-tab-pane>

        <el-tab-pane label="死之羽">
            <select-single-artifact
                :data="feather"
                :value="value[1]"
                @input="changeFeather"
            >
            </select-single-artifact>
        </el-tab-pane>

        <el-tab-pane label="时之沙">
            <select-single-artifact
                :data="sand"
                :value="value[2]"
                @input="changeSand"
            >
            </select-single-artifact>
        </el-tab-pane>

        <el-tab-pane label="空之杯">
            <select-single-artifact
                :data="cup"
                :value="value[3]"
                @input="changeCup"
            >
            </select-single-artifact>
        </el-tab-pane>

        <el-tab-pane label="理之冠">
            <select-single-artifact
                :data="head"
                :value="value[4]"
                @input="changeHead"
            >
            </select-single-artifact>
        </el-tab-pane>
    </el-tabs>
</template>

<script>
import SelectSingleArtifact from "./SelectSingleArtifact";

import { mapState } from "vuex";

export default {
    name: "SelectArtifact",
    components: {
        SelectSingleArtifact,
    },
    props: {
        value: {
            type: Array,
        }
    },
    data: function() {
        return {
            // arts: [-1, -1, -1, -1, -1],
        }
    },
    methods: {
        changeFlower(v) {
            this.$set(this.value, 0, v);
        },
        changeFeather(v) {
            this.$set(this.value, 1, v);
        },
        changeSand(v) {
            this.$set(this.value, 2, v);
        },
        changeCup(v) {
            this.$set(this.value, 3, v);
        },
        changeHead(v) {
            this.$set(this.value, 4, v);
        }
    },
    computed: {
        ...mapState([
            "flower",
            "feather",
            "sand",
            "cup",
            "head",
        ])
    }
}
</script>