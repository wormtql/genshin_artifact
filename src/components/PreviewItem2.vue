<template>
    <!-- <el-card> -->
    <div class="root">
        <h5>主属性：</h5>
        <el-alert
            v-for="key in filteredPrimaryTags"
            :key="key"
            :title="chsPrimaryTag(key) + '：' + value.primary[key]"
            :closable="false"
            class="item"
        >
            
        </el-alert>

        <h5>副属性：</h5>
        <el-alert
            v-for="key in filteredSecondaryTags"
            :key="key"
            :title="displayTag(key, value.secondary[key])"
            :closable="false"
            class="item"
        >
            
        </el-alert>
    <!-- </el-card> -->
    </div>
</template>

<script>
import { chsPrimaryTag, chsSecondaryTag } from "@/common/chs";
import { displayTag } from "@/utils";

export default {
    name: "PreviewItem2",
    props: {
        value: {
            type: Object,
        }
    },
    methods: {
        chsPrimaryTag,
        chsSecondaryTag,
        displayTag,
    },
    computed: {
        filteredPrimaryTags() {
            let temp = Object.keys(this.value.primary);
            return temp.filter(item => chsPrimaryTag(item));
        },

        filteredSecondaryTags() {
            let temp = Object.keys(this.value.secondary);
            return temp.filter(item => chsSecondaryTag(item));
        },
    }
}
</script>

<style scoped>
/* .root {
    border-left: 5px solid #67C23A;
    padding-left: 16px;
} */

.item {
    /* font-size: 0.7rem;
    margin: 0;
    padding: 4px;
    font-weight: bold; */
    /* margin-right: 8px; */
    margin-bottom: 8px;
}
</style>