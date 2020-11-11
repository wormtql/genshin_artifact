<template>
    <el-tabs type="border-card">
        <el-tab-pane
            v-for="(cs, element) in characterPreset"
            :key="element"
            :label="cs.chs"
        >
            <el-collapse>
                <el-collapse-item
                    v-for="(c, name) in cs.characters"
                    :key="name"
                    :title="c.chs"
                >
                    <el-radio-group
                        :value="value"
                        @input="$emit('input', $event)"
                    >
                        <el-radio-button
                            v-for="preset in c.presets"
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
                    v-for="(cus, name) in customedCharacters"
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
import { mapState } from "vuex";
import { characterPreset } from "@/common/basic";

export default {
    name: "SelectCharacter",
    props: {
        value: {
            type: String
        }
    },
    data: function() {
        return {
            characterPreset,
        }
    },
    computed: {
        ...mapState([
            // "selectedCharacter",
            // "selectedWeapon",
            // "selectedWeaponAttribute",
            // "selectedCharacterAttribute",
            // "customWeapon",
            // "customCharacter",
            "customedCharacters",
            // "customedWeapons",
            // "customedTargets",

            // "selectedTargetFunction",
        ])
    }
}
</script>