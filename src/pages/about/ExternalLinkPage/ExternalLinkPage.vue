<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>友情链接</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <div class="container">
            <div
                class="link-item"
                v-for="link in links"
                :key="link.name"
            >
                <h3 class="link-title">{{ link.name }}</h3>
                <div class="link-tags">
                    <span
                        class="link-tag fs-12"
                        v-for="(tag, index) in link.tags"
                        :key="index"
                    >{{ tag }}</span>
                </div>
                <div class="link-footer">
                    <a class="el-button link-button" :href="link.url" target="_blank">
                        <i class="el-icon-connection" style="margin-right: 4px"></i>
                        {{ link.url }}
                    </a>
                </div>
            </div>
            
        </div>
    </div>
</template>

<script>
import links from "./links";

export default {
    name: "ExternalLinkPage",
    created: function () {
        this.links = links;
    },
    methods: {
        navigateTo(url) {
            window.open(url, "_blank");
        }
    }
    // links,
}
</script>

<style scoped>
.link-button {
    text-decoration: none;
}

.container {
    display: flex;
    flex-wrap: wrap;
    align-content: stretch;
}

.link-item {
    border-radius: 3px;
    box-shadow: 0 0 10px 1px #00000011;
    padding: 20px;
    width: 28%;
    margin-right: 20px;
}

.link-title {
    margin: 0;
    margin-bottom: 10px;
    color: #123456;
}

.link-tags {
    display: flex;
    flex-wrap: wrap;
}

.link-tag {
    background: #409eff;
    color: white;
    height: 24px;
    border-radius: 12px;
    padding: 0 8px;
    line-height: 24px;
    margin: 8px 8px 0 0;
}

.link-footer {
    margin-top: 24px;
}
</style>