<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>友情链接</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-row :gutter="16">
            <el-col
                :xs="24"
                :sm="8"
                v-for="link in links"
                :key="link.name"
            >
                <el-card
                    style="margin-bottom: 16px"
                >
                    <!-- <div s>
                    </div> -->
                    <span slot="header" class="link-title">{{ link.name }}</span>

                    <div class="words" v-if="link.word">
                        <span class="quote">“</span>
                        <span class="content">{{ link.word }}</span>
                    </div>

                    <div class="link-tags">
                        <span
                            class="link-tag fs-12"
                            v-for="(tag, index) in link.tags"
                            :key="index"
                        >{{ tag }}</span>
                    </div>
                    <div class="link-footer">
                        <a
                            v-for="(url, caption) in link.url"
                            :key="caption"
                            class="el-button link-button"
                            :href="url"
                            target="_blank"
                        >
                            <i class="el-icon-connection" style="margin-right: 4px"></i>
                            {{ caption }}
                        </a>
                    </div>
                </el-card>
            </el-col>
            
        </el-row>
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

<style scoped lang="scss">
.words {
    .quote {
        font-size: 3rem;
        padding: 0 8px;
        vertical-align: baseline;
        position: relative;
        line-height: 16px;
        top: 18px;
    }

    .content {
        font-size: 14px;
        vertical-align: bottom;
    }

    background: #f4f4f5;
    color: #909399;
    padding: 16px;
    border-radius: 3px;
    margin-bottom: 8px;
}

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