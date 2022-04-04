<template>
    <div class="page-root">
        <el-breadcrumb>
            <el-breadcrumb-item>主页</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <p
            class="title"
        >
            <span class="mona">{{ webTitle }}</span>
            V{{ version }}
        </p>
        
        <!-- <p style="padding: 0; margin: 0; font-size: 10px;">该网站处处透露着贫穷，欢迎有志者参与开发（美术等）</p> -->

<!--        <migrate-notification-->
<!--            v-if="needMigrate"-->
<!--            style="margin-bottom: 16px"-->
<!--        ></migrate-notification>-->

        <el-alert
            class="hidden-sm-and-up"
            title="使用PC访问以启用全部功能，当前仅能查看信息页"
            type="error"
            style="margin-bottom: 16px"
            :closable="false"
        ></el-alert>

        <el-row :gutter="16">
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item
                    text="伤害计算器/单人配装"
                    icon="calculator"
                    url="/calculate"
                ></use-case-item>
            </el-col>
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item
                    text="队伍配装"
                    icon="bell"
                    url="/team-optimization"
                ></use-case-item>
            </el-col>
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item
                    text="圣遗物潜力"
                    icon="thumbs-up"
                    url="/potential"
                ></use-case-item>
            </el-col>
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item
                    text="导出工具"
                    icon="chart-pie"
                ></use-case-item>
            </el-col>
        </el-row>

        <el-card class="mb16" shadow="never">
            <p class="big-card-title">开源地址</p>
            <el-row :gutter="16">
                <el-col :xs="24" :sm="12">
                    <div
                        class="source-code-item"
                        @click="newPage(links.frontendProject)"
                    >
                        <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>
                        <p class="item-title">Mona</p>
                    </div>
                </el-col>
                <el-col :xs="24" :sm="12">
                    <div
                        class="source-code-item"
                        @click="newPage(links.yasProject)"
                    >
                        <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>
                        <p class="item-title">YAS</p>
                    </div>
                </el-col>
            </el-row>
        </el-card>

        <el-row :gutter="16">
            <el-col :xs="24" :sm="12">
                <el-card class="mb16" shadow="never">
                    <p class="card-title">反馈</p>
                    <el-button @click="newPage(links.issue)" class="data-source">
                        github Issue
                        <font-awesome-icon :icon="['fab', 'github']"></font-awesome-icon>
                    </el-button>
                    <el-button
                        class="data-source"
                        @click="newPage('https://qm.qq.com/cgi-bin/qm/qr?k=yQaJgPzRmBgEXXk1uiqNbq7CIrq-0biW&jump_from=webapi')"
                    >
                        QQ群(801106595)
                        <font-awesome-icon icon="comment"></font-awesome-icon>
                    </el-button>
                </el-card>
            </el-col>

            <el-col :xs="24" :sm="12">
                <el-card class="mb16" shadow="never">
                    <p class="card-title">数据来源</p>
                    <el-button @click="newPage(links.wiki)" class="data-source">
                        bilibili原神wiki
                        <font-awesome-icon icon="database"></font-awesome-icon>
                    </el-button>
                    <el-button @click="newPage(links.wiki2)" class="data-source">
                        Genshin Impact Wiki | Fandom
                        <font-awesome-icon icon="database"></font-awesome-icon>
                    </el-button>
                    <el-button @click="newPage(links.wiki3)" class="data-source">
                        旅行者创作平台-观测枢-原神wiki
                        <font-awesome-icon icon="database"></font-awesome-icon>
                    </el-button>
                </el-card>
            </el-col>
        </el-row>

        <el-row :gutter="16">
            <el-col :xs="24" :sm="12">
                <el-card class="mb16" shadow="never">
                    <p class="card-title">圣遗物导出工具</p>
                    <el-button
                        @click="newPage(links.yasDownloadGithub)"
                        type="primary"
                        class="data-source"
                    >
                        YAS下载地址
                        <i class="el-icon-aim"></i>
                    </el-button>
                    <el-button
                        @click="$router.push('/help/export-tools')"
                        class="data-source"
                    >
                        导出工具大全
                        <i class="el-icon-aim"></i>
                    </el-button>
                </el-card>
            </el-col>
            <el-col :xs="24" :sm="12">
                <el-card class="mb16" shadow="never">
                    <p class="card-title">开发者文档</p>
                    <a class="el-button no-deco" :href="links.doc" target="_blank">
                        文档
                        <i class="el-icon-connection"></i>
                    </a>
                </el-card>
            </el-col>
        </el-row>

        <el-card style="margin-bottom: 16px" shadow="never">
            <p class="card-title">请莫娜吃饭</p>
            <el-alert
                :closable="false"
                title="才...才不是因为交不起服务器费呢。"
            ></el-alert>
            <!-- 暂未开通 -->
            <div class="pay">
                <span>微信支付：</span>
                <img src="./wechat.png">
            </div>
            <div class="pay">
                <span>支付宝：</span>
                <img src="./alipay.png">
            </div>
        </el-card>
    </div>
</template>

<script>
import links from "@const/links"

import MigrateNotification from "./MigrateNotification"
import UseCaseItem from "./UseCaseItem"

export default {
    name: "IntroPage",
    components: {
        MigrateNotification,
        UseCaseItem,
        // NokNok,
    },
    created: function() {
        this.links = links;

        this.version = process.env.MONA_VERSION;
        this.webTitle = process.env.MONA_TITLE;
        this.needMigrate = process.env.MONA_NEED_MIGRATE;
        this.buildDate = process.env.MONA_BUILD_DATE;

        this.host = location.hostname;
    },
    methods: {
        navigateTo(des) {
            this.$router.replace(des);
        },
        newPage(des) {
            window.open(des, "_blank");
        }
    }
}
</script>

<style lang="scss" scoped>
.page-root {
    height: 100%;
    overflow: auto;

    -ms-overflow-style: none;
}

.page-root::-webkit-scrollbar {
    display: none;
}

.build-info {
    font-size: 12px;

}

.mb16 {
    margin-bottom: 16px;
}

.pay {
    display: flex;
    align-items: center;
    margin-top: 20px;

    span {
        width: 100px;
    }

    img {
        width: 96px;
        height: 96px;
        display: block;
    }
}

.title {
    font-size: 3rem;
    margin: 0;
    margin-bottom: 24px;
    padding: 0;
}

.item {
    cursor: pointer;
}

.item-title {
    font-weight: bold;
    font-size: 1.2rem;
    padding: 0;
    margin: 16px 0 0 0;
}

.icon {
    font-size: 1.5rem;
}

.card-title {
    font-size: 1.2rem;
    margin: 0 0 16px 0;
}

.big-card-title {
    margin: 0;
    padding: 0 0 16px 0;
    font-size: 1.5rem;
}

.data-source {
    margin: 0;
    margin-right: 10px;
    margin-top: 10px;
}

.mona {
    background: rgb(86,72,132);
    border-radius: 3px;
    color: white;
    padding: 0 8px;
}

.source-code-item {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 15vh;
    border: 1px solid #00000011;
    cursor: pointer;

    &:hover {
        background: rgb(251, 249, 255);
    }
}
</style>