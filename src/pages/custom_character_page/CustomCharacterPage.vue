<template>
    <div>
        <new-item-dialog :show="showNewItemDialog" title="自定义角色"
            :usedNames="usedCharacterNames"
            @cancel="showNewItemDialog=false"
            @confirm="onDialogConfirm"
        >
        </new-item-dialog>
        
        <el-breadcrumb>
            <el-breadcrumb-item>自定义角色</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-button @click="showNewItemDialog = true" type="primary" icon="el-icon-plus"
            style="margin-bottom: 16px"
        >添加</el-button>

        <el-tabs
            type="border-card"
            tab-position="left"
            v-if="usedCharacterNames.length !== 0"
        >
            
            <el-tab-pane
                v-for="(value, name) in customedCharacters"
                :key="name"
                :label="name"
                :name="name"
            >
                
                <!-- <el-button type="primary">应用</el-button> -->

                <preview-item2 :value="value"></preview-item2>

                <el-button style="margin-top: 16px" @click="deleteCustomCharacter(name)">删除</el-button>
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
        <p v-else>未添加自定义角色</p>
    </div>
</template>

<script>
import NewItemDialog from "@/components/NewItemDialog";
import PreviewItem2 from "@/components/PreviewItem2";

import { mapState } from "vuex";

export default {
    name: "CustomCharacterPage",
    components: {
        NewItemDialog,
        PreviewItem2,
    },
    data: function() {
        return {
            showNewItemDialog: false,
        }
    },
    methods: {
        onDialogConfirm(item) {
            // window.console.log(item);
            this.showNewItemDialog = false;
            this.$store.commit("addCustomedCharacter", item);
        },
        deleteCustomCharacter(name) {
            this.$store.commit("deleteCustomedCharacter", name);
        }
    },
    computed: {
        ...mapState([
            // "selectedWeapon",
            // "selectedCharacter",

            // "selectedWeaponAttribute",
            // "selectedCharacterAttribute",

            // "customedWeapons",
            "customedCharacters",

            // "selectedCustomWeapon",
            // "selectedCustomCharacter",
        ]),
        usedCharacterNames() {
            return Object.keys(this.customedCharacters);
        }
    }
}
</script>