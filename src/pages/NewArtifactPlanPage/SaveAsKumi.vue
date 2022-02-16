<template>
    <div>
        <p class="title">选择收藏夹</p>
        <div class="select-dir">
            <div
                class="dir-item"
                v-for="dir in directories"
                :key="dir.id"
            >
                <el-checkbox ref="checkbox" :x-id="dir.id">{{ dir.title }}</el-checkbox>
            </div>
        </div>

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
                @click="handleConfirm"
            >确定</el-button>

        </div>
    </div>
</template>

<script>
import { mapGetters } from "vuex"

export default {
    name: "SaveAsKumi",
    data() {
        return {
            name: ""
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
            if (this.name === "") {
                this.$message.error("名称不能为空")
                return
            }
            const dirIds = this.getCheckedDirIds()

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