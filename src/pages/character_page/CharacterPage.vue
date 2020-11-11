<template>
    <div>
        <new-item-dialog :show="showNewItemDialog" :title="newItemDialogTitle"
            :type="customType"
            @cancel="showNewItemDialog=false"
            @confirm="onDialogConfirm"
        >
        </new-item-dialog>

        <el-breadcrumb>
            <el-breadcrumb-item>角色</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row :gutter="16">
            <el-col :span="17">
                <!-- <el-switch
                    v-model="isCustom"
                    active-text="自定义面板"
                    inactive-text="使用预设"
                ></el-switch> -->

                <h3>摘要</h3>
                <el-row>
                    <el-col :span="12">
                        <p>已选角色属性：</p>
                        <preview-item :value="selectedCharacterAttribute">
                        </preview-item>
                    </el-col>
                    <el-col :span="12">
                        <p>已选武器属性：</p>
                        <preview-item :value="selectedWeaponAttribute">
                        </preview-item>
                    </el-col>
                </el-row>

                <h3>角色</h3>
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
                                    :value="selectedCharacter"
                                    @input="onCharacterPresetClicked"
                                    :disabled = "isCustom"
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

                    <el-tab-pane label="高级">
                        <div>
                            <el-button circle icon="el-icon-plus" size="medium"
                                style="margin-bottom: 16px" type="primary"
                                @click="onAddCustomCharacterClicked"
                            >
                            </el-button>
                        </div>

                        <!-- <el-radio-group
                            :value="selectedCustomCharacter"
                            @input="onCustomCharacterClicked"
                        >
                            <el-radio-button
                                v-for="(value, name) in customedCharacters"
                                :key="name"
                                :label="name"
                            ></el-radio-button>
                        </el-radio-group> -->
                        <el-tabs
                            type="border-card"
                            tab-position="left"
                            
                        >
                            <el-tab-pane
                                v-for="(value, name) in customedCharacters"
                                :key="name"
                                :label="name"
                                :name="name"
                            >
                                <el-button>删除</el-button>
                                <el-button type="primary">应用</el-button>

                                <preview-item2 :value="value"></preview-item2>
                                <!-- <el-row>
                                    <el-col :span="12">
                                        <preview-item :value="value"></preview-item>
                                    </el-col>
                                    <el-col :span="12">
                                        
                                    </el-col>
                                </el-row> -->
                                <!-- {{ value }} -->
                                
                                
                            </el-tab-pane>
                        </el-tabs>
                    </el-tab-pane>
                </el-tabs>

                <h3>武器</h3>
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
                                    :value="selectedWeapon"
                                    @input="onWeaponPresetClicked"
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

                    <el-tab-pane label="高级">
                        <div>
                            <el-button circle icon="el-icon-plus" size="medium"
                                style="margin-bottom: 16px" type="primary"
                                @click="onAddCustomWeaponClicked"
                            >
                            </el-button>
                        </div>

                        <el-radio-group
                            :value="selectedCustomWeapon"
                            @input="onCustomWeaponClicked"
                        >
                            <el-radio-button
                                v-for="(value, name) in customedWeapons"
                                :key="'weapon' + name"
                                :label="name"
                            ></el-radio-button>
                        </el-radio-group>
                    </el-tab-pane>
                </el-tabs>
            </el-col>
            <el-col :span="7">
                <h3>面板预览</h3>
                <attribute-panel :panel="currentPanel"
                    :disabled="!isCustom"
                ></attribute-panel>
                
            </el-col>
        </el-row>
        
    </div>
</template>

<script>
import AttributePanel from "@/components/AttributePanel";
import { characterPreset, weaponPreset } from "@/common/basic";
import { compose } from "genshin_panel";
import PreviewItem from "./PreviewItem";
import PreviewItem2 from "@/components/PreviewItem2";
import NewItemDialog from "@/components/NewItemDialog";
import { mapState } from "vuex";

export default {
    name: "CharacterPage",
    components: {
        AttributePanel,
        PreviewItem,
        NewItemDialog,
        PreviewItem2
    },
    // mounted: function() {
    //     window.console.log(compose("keqing-70-0", "heijian-70-0", []));
    // },
    data: function() {
        return {
            publicPath: process.env.BASE_URL,
            characterPreset,
            weaponPreset,

            attribute2: Object.assign({}, this.$store.state.attribute),

            // selectedWeapon: "heijian-70-0",
            // selectedCharacter: "keqing-70-0",
            isCustom: false,

            showNewItemDialog: false,
            newItemDialogTitle: "",
            customType: "",
        }
    },
    methods: {
        // 点击角色预设
        onCharacterPresetClicked(presetName) {
            this.$store.commit("setSelectedCharacter", presetName);
        },
        // 点击武器预设
        onWeaponPresetClicked(presetName) {
            this.$store.commit("setSelectedWeapon", presetName);
        },
        onCustomWeaponClicked(name) {
            this.$store.commit("setSelectedCustomWeapon", name);
        },
        onCustomCharacterClicked(name) {
            this.$store.commit("setSelectedCustomCharacter", name);
            // window.console.log(this.selectedCharacterAttribute);
        },
        // 点击添加自定义角色
        onAddCustomCharacterClicked() {
            this.showNewItemDialog = true;
            this.newItemDialogTitle = "自定义角色属性";
            this.customType = "character";
        },
        onAddCustomWeaponClicked() {
            this.showNewItemDialog = true;
            this.newItemDialogTitle = "自定义武器属性";
            this.customType = "weapon";
        },
        onDialogConfirm(item) {
            // window.console.log(item);
            this.showNewItemDialog = false;
            if (this.customType === "character") {
                this.$store.commit("addCustomedCharacter", item);
                // this.$store.commit("setSelectedCharacterAttribute", item.item);
            } else {
                this.$store.commit("addCustomedWeapon", item);
            }
            // window.console.log(item);
        }
    },
    // created: function() {
    //     this.updateAttribute();
    // },
    computed: {
        // selectedWeaponAttribute() {
        //     return getWeaponAttribute(this.selectedWeapon);
        // },
        // selectedCharacterAttribute() {
        //     return getCharacterAttribute(this.selectedCharacter);
        // },
        currentPanel() {
            let attribute = compose(this.selectedCharacterAttribute, this.selectedWeaponAttribute);
            return attribute;
        },

        ...mapState([
            "selectedWeapon",
            "selectedCharacter",

            "selectedWeaponAttribute",
            "selectedCharacterAttribute",

            "customedWeapons",
            "customedCharacters",

            "selectedCustomWeapon",
            "selectedCustomCharacter",
        ])
    }
}
</script>

<style scoped>

</style>