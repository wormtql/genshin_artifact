<template>
    <el-tabs type="border-card">
        <el-tab-pane
            v-for="(weapons, wType) in weaponPreset"
            :key="wType"
            :label="weapons.chs"
        >
            <el-collapse>
                <el-collapse-item
                    v-for="(weapon, name) in weapons.weapons"
                    :key="name"
                    :title="weapon.chs"
                >
                    <el-radio-group
                        :value="value"
                        @input="$emit('input', $event)"
                    >
                        <el-radio-button
                            v-for="preset in weapon.presets"
                            :key="preset.value"
                            :label="preset.value"
                        >
                            {{ preset.title }}
                        </el-radio-button>
                    </el-radio-group>
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
                    v-for="(cus, name) in customedWeapons"
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
import { weaponPreset } from "@/common/basic";
import { mapState } from "vuex";

export default {
    name: "SelectWeapon",
    props: {
        value: {
            type: String,
        }
    },
    data: function() {
        return {
            weaponPreset,
        }
    },
    computed: {
        ...mapState([
            "customedWeapons"
        ])
    }
}
</script>