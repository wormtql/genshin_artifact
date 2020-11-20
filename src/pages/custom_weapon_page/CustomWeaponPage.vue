<template>
    <div>
        <new-item-dialog :show="showNewItemDialog" title="自定义武器"
            :usedNames="usedWeaponNames"
            @cancel="showNewItemDialog=false"
            @confirm="onDialogConfirm"
            primaryName="基础属性"
            secondaryName="副属性"
        >
        </new-item-dialog>

        <el-dialog :visible.sync="exportJsonDialogVisible" title="导出JSON">
            <p style="max-height: 300px; overflow: auto">{{ json }}</p>
            <span slot="footer">
                <el-button class="clip" @click="clipJson">复制</el-button>
            </span>
        </el-dialog>
        
        <el-breadcrumb>
            <el-breadcrumb-item>自定义武器</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-alert :closable="false" style="margin-bottom: 16px" show-icon>
            <template #title>
                由于武器预设并不包含所有武器，且不包含特效效果，如果一定要追求准确，可以在此添加自定义武器<br>
                “基础属性”一栏填写武器的基础攻击力<br>
                “副属性”一栏填写武器副属性以及特效等
            </template>
        </el-alert>

        <div class="bar">
            <el-button @click="showNewItemDialog = true" type="primary" icon="el-icon-plus"
            >添加</el-button>

            <div style="vertical-align: top">
                <el-button @click="exportJson">导出Json</el-button>
                <el-button @click="importJson">导入Json</el-button>
            </div>
        </div>
        

        <el-tabs
            type="border-card"
            tab-position="left"
            v-if="usedWeaponNames.length !== 0"
        >
            
            <el-tab-pane
                v-for="(value, name) in customedWeapons"
                :key="name"
                :label="name"
                :name="name"
            >
                
                <!-- <el-button type="primary">应用</el-button> -->

                <preview-item2 :value="value"></preview-item2>

                <el-button style="margin-top: 16px" @click="deleteCustomWeapon(name)">删除</el-button>
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
        <p v-else>未添加自定义武器</p>
    </div>
</template>

<script>
import NewItemDialog from "@/components/NewItemDialog";
import PreviewItem2 from "@/components/PreviewItem2";

import { mapState } from "vuex";

import Clipboard from "clipboard";

export default {
    name: "CustomWeaponPage",
    components: {
        NewItemDialog,
        PreviewItem2,
    },
    data: function() {
        return {
            showNewItemDialog: false,

            exportJsonDialogVisible: false,
            json: "",
        }
    },
    methods: {
        onDialogConfirm(item) {
            // window.console.log(item);
            this.showNewItemDialog = false;
            this.$store.commit("addCustomedWeapon", item);
        },
        deleteCustomWeapon(name) {
            this.$store.commit("deleteCustomedWeapon", name);
        },
        exportJson() {
            this.json = JSON.stringify(this.customedWeapons);
            this.exportJsonDialogVisible = true;
        },
        importJson() {
            this.$prompt("输入JSON", "导入JSON", {
                confirmButtonText: "确定",
                cancelButtonText: "取消"
            }).then(({ value }) => {
                if (value === "") {
                    return;
                }
                return new Promise((resolve, reject) => {
                    let obj = JSON.parse(value);
                    if (typeof obj !== "object") {
                        reject("wrong_format");
                    }

                    for (let key in obj) {
                        this.$store.commit("addCustomedWeapon", {
                            name: key,
                            item: obj[key],
                        });
                    }
                })
            }).catch((e) => {
                if (e !== "cancel") {
                    this.$message.error("请输入正确的json字符串");
                }
            })
        },
        clipJson() {
            let s = this.json;
            let clipboard = new Clipboard(".clip", {
                text: function() {
                    return s;
                }
            });
            clipboard.on("success", () => {
                this.$message({ message: "复制成功", type: "success" });
                clipboard.destroy();
            });
            clipboard.on("error", () => {
                this.$message({ message: "复制失败", type: "error" });
                clipboard.destroy();
            });
        },
    },
    computed: {
        ...mapState([
            // "selectedWeapon",
            // "selectedCharacter",

            // "selectedWeaponAttribute",
            // "selectedCharacterAttribute",

            "customedWeapons",
            // "customedCharacters",

            // "selectedCustomWeapon",
            // "selectedCustomCharacter",
        ]),
        usedWeaponNames() {
            return Object.keys(this.customedWeapons);
        }
    }
}
</script>

<style scoped>
.bar {
    display: flex;
    align-items: center;
    justify-content: space-between;

    margin-bottom: 16px;
}
</style>