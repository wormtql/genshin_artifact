<template>
    <div>
        <new-item-dialog :show="showNewItemDialog" title="自定义角色"
            :usedNames="usedCharacterNames"
            @cancel="showNewItemDialog=false"
            @confirm="onDialogConfirm"
            primaryName="基础属性"
            secondaryName="突破加成属性"
        >
        </new-item-dialog>

        <el-dialog :visible.sync="exportJsonDialogVisible" title="导出JSON">
            <p style="max-height: 300px; overflow: auto">{{ json }}</p>
            <span slot="footer">
                <el-button class="clip" @click="clipJson">复制</el-button>
            </span>
        </el-dialog>
        
        <el-breadcrumb>
            <el-breadcrumb-item>自定义角色</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-alert :closable="false" style="margin-bottom: 16px" show-icon>
            <template #title>
                由于角色预设并不包含所有角色等级，如果一定要追求准确，可以在此添加自定义角色<br>
                “基础属性”一栏填写角色的基础攻击力、防御力、生命值（除去所有装备和圣遗物加成）<br>
                “突破加成属性”一栏填写角色的突破加成属性（如果是暴击率，则要减去初始暴击率0.05）
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

import Clipboard from "clipboard";

export default {
    name: "CustomCharacterPage",
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
            this.$store.commit("addCustomedCharacter", item);
        },
        deleteCustomCharacter(name) {
            this.$store.commit("deleteCustomedCharacter", name);
        },
        exportJson() {
            this.json = JSON.stringify(this.customedCharacters);
            this.exportJsonDialogVisible = true;
        },
        importJson() {
            this.$prompt("输入JSON", "导入JSON", {
                confirmButtonText: "确定",
                cancelButtonText: "取消"
            }).then(({ value }) => {
                return new Promise((resolve, reject) => {
                    let obj = JSON.parse(value);
                    if (typeof obj !== "object") {
                        reject("wrong_format");
                    }

                    for (let key in obj) {
                        this.$store.commit("addCustomedCharacter", {
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

<style scoped>
.bar {
    display: flex;
    align-items: center;
    justify-content: space-between;

    margin-bottom: 16px;
}
</style>