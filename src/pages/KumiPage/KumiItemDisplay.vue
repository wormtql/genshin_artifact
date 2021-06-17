<template>
    <div>
        <h2>{{ node.label }}</h2>
    </div>
</template>

<script>
import getTreeNode from "@/store/utils/getTreeNode";

export default {
    name: "KumiItemDisplay",
    props: ["dirId"],
    computed: {
        node() {
            return getTreeNode(this.$store.state.kumi["tree"], node => node.id === this.dirId && node.type === "dir");
        }
    }
}
</script>