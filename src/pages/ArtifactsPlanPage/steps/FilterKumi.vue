<template>
    <div>
        <el-alert
            title="选中的套装将被排除"
            :closable="false"
            style="margin-bottom: 16px"
        ></el-alert>

        <el-tree
            :data="treeData"
            show-checkbox
            ref="tree"
        >
            <span slot-scope="{ node, data }">
                <span>
                    <i class="el-icon-folder" v-if="!node.expanded && data.type === 'dir'"></i>
                    <i class="el-icon-folder-opened" v-if="node.expanded && data.type === 'dir'"></i>
                    <i class="el-icon-s-grid" v-if="data.type === 'data'"></i>
                    {{ node.label }}
                </span>
            </span>
        </el-tree>
    </div>
</template>

<script>
export default {
    name: "FilterKumi",
    computed: {
        treeData() {
            return this.$store.state.kumi["tree"].children;
        }
    },
    methods: {
        getExcludedId() {
            let nodes = this.$refs.tree.getCheckedNodes(true);
            
            let ret = new Set();
            for (let node of nodes) {
                if (node && node.data) {
                    for (let id of node.data.ids) {
                        ret.add(id);
                    }
                }
            }

            // console.log(ret);
            return ret;
        }
    }
}
</script>