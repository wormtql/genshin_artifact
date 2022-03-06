<template>
    <div>
        <p class="title">选择收藏夹</p>
        <!-- <div class="select-dir">
            <div
                class="dir-item"
                v-for="dir in directories"
                :key="dir.id"
            >
                <el-checkbox ref="checkbox" :x-id="dir.id">{{ dir.title }}</el-checkbox>
            </div>
        </div> -->

        <el-checkbox-group
            v-model="checkList"
            :min="1"
        >
            <el-checkbox
                v-for="dir in directories"
                :key="dir.id"
                :label="dir.id"
                border
                class="dir-item"
            >{{ dir.title }}</el-checkbox>
        </el-checkbox-group>

        <p class="title">名称</p>
        <el-input
            v-model="name"
            placeholder="输入名称"
        ></el-input>

        <div class="buttons" style="text-align: right; margin-top: 32px">
<!--            <el-button-->
<!--            >取消</el-button>-->
            <el-button
                type="primary"
                :disabled="name === ''"
                @click="handleConfirm"
            >确定</el-button>

        </div>
    </div>
</template>

<script>
import { mapGetters } from "vuex"

export default {
    name: "SaveAsKumi",
    props: {
        defaultName: {
            type: String,
            default: "",
        }
    },
    data() {
        return {
            name: this.defaultName,
            checkList: [0],
        }
    },
    watch: {
        defaultName(newName) {
            this.name = newName
        }
    },
    methods: {
        getCheckedDirIds() {
            const components = this.$refs["checkbox"]
            if (!components) {
                return []
            }

            let set = new Set()
            for (let checkbox of components) {
                // console.log(checkbox)
                if (checkbox.isChecked) {
                    set.add(checkbox.$attrs["x-id"])
                }
            }

            return Array.from(set)
        },

        handleConfirm() {
            // const dirIds = this.getCheckedDirIds()
            const dirIds = this.checkList

            this.$emit("confirm", {
                dirIds,
                name: this.name
            })
        }
    },
    computed: {
        ...mapGetters("kumi", [
            "directories",
            "dirNames"
        ]),
    }
}
</script>

<style scoped>

</style>
