<template>
    <el-tabs type="border-card">
        <el-tab-pane label="角色预设">
            <el-collapse
                :value="value"
                @input="onInput"
                accordion
            >
                <el-collapse-item
                    v-for="commonPreset in plansPreset.character"
                    :key="commonPreset.value"
                    :name="commonPreset.value"
                >
                    <template #title>
                        <span style="margin-right: 8px">{{ commonPreset.name }}</span>
                        <i class="el-icon-check" v-show="commonPreset.value === value"
                            style="color: green; font-weight: bold"
                        ></i>
                    </template>
                    <p>{{ commonPreset.description }}</p>
                    <ul v-if="commonPreset.tags">
                        <li
                            v-for="(tag, index) in commonPreset.tags"
                            :key="index"
                        >
                            {{ tag }}
                        </li>
                    </ul>
                </el-collapse-item>
            </el-collapse>
        </el-tab-pane>
        <el-tab-pane label="基础属性">
            <el-collapse
                :value="value"
                @input="onInput"
                accordion
            >
                <el-collapse-item
                    v-for="commonPreset in plansPreset.common"
                    :key="commonPreset.value"
                    
                    :name="commonPreset.value"
                >
                    <template #title>
                        <span style="margin-right: 8px">{{ commonPreset.name }}</span>
                        <i class="el-icon-check" v-show="commonPreset.value === value"
                            style="color: green; font-weight: bold"
                        ></i>
                    </template>
                    <p>{{ commonPreset.description }}</p>
                    <ul v-if="commonPreset.tags">
                        <li
                            v-for="(tag, index) in commonPreset.tags"
                            :key="index"
                        >
                            {{ tag }}
                        </li>
                    </ul>
                </el-collapse-item>
            </el-collapse>
        </el-tab-pane>

        <el-tab-pane>
            <template #label>
                <i class="el-icon-s-tools"></i>
                自定义
            </template>

            <el-radio-group :value="value" @input="$emit('input', $event)">
                <el-radio-button
                    v-for="(cus, name) in customedTargets"
                    :key="name"
                    :label="name"
                >
                    {{ name }}
                </el-radio-button>
            </el-radio-group>
        </el-tab-pane>
    </el-tabs>
</template>

<script>
import { plans as plansPreset } from "@/common/target";
import { mapState } from "vuex";

export default {
    name: "SelectTarget",
    props: {
        value: {
            type: String
        }
    },
    data: function() {
        return {
            plansPreset,
        }
    },
    methods: {
        onInput(e) {
            if (e !== "") {
                this.$emit("input", e);
            }
        }
    },
    computed: {
        ...mapState([
            "customedTargets",
        ])
    }
}
</script>