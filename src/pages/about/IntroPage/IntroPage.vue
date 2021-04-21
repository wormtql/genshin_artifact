<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>主页</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <p
            class="title"
            style="margin-bottom: 24px"
        >
            <span class="mona">{{ webTitle }}</span>
            V{{ version }}
        </p>
        <!-- <p style="padding: 0; margin: 0; font-size: 10px;">该网站处处透露着贫穷，欢迎有志者参与开发（美术等）</p> -->

        <el-alert
            class="hidden-sm-and-up"
            title="使用PC访问以启用全部功能，当前仅能查看信息页"
            type="error"
            style="margin-bottom: 16px"
            :closable="false"
        ></el-alert>

        <div class="update-banner">
            <p style="font-size: 24px; margin: 0;">V{{ version }}更新内容</p>
            <ul>
                <li
                    v-for="(item, index) in lastestVersion.changes"
                    :key="index"
                >{{ item }}</li>
            </ul>
        </div>

        <el-card class="mb16">
            <p class="big-card-title">用例</p>
            <ul>
                <li>我的圣遗物最高能到多少暴击伤害？</li>
                <li>我的圣遗物最高能堆到多少生命值？</li>
                <li>如何使单次打出的伤害最高？</li>
                <li>如何使期望伤害最高？</li>
                <li>暴击、爆伤、攻击力要多少比例？</li>
                <li>诺艾尔的防御收益到底怎么样</li>
                <li>多方向圣遗物评分与潜力计算</li>
                <li>...</li>
            </ul>
            <!-- <p>以上都不是问题，一次输入圣遗物，即可自动计算</p> -->
        </el-card>

        <el-card class="mb16">
            <p class="big-card-title">开源地址</p>
            <el-alert
                title="该工具全部免费且开源"
                :closable="false"
                style="margin-bottom: 16px"
            ></el-alert>
            <el-row :gutter="16">
                <el-col :xs="24" :sm="12">
                    <el-card
                        :body-style="{textAlign: 'center'}"
                        @click.native="newPage(links.frontendProject)"
                        class="item mb16"
                        shadow="hover"
                    >
                        <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>
                        <p class="item-title">前端</p>
                    </el-card>
                </el-col>
                <el-col :xs="24" :sm="12">
                    <el-card
                        :body-style="{textAlign: 'center'}"
                        @click.native="newPage(links.panelProject)" class="item"
                        shadow="hover"
                    >
                        <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>
                        <p class="item-title">面板计算器（npm包）</p>
                    </el-card>
                </el-col>
            </el-row>
        </el-card>

        <el-row :gutter="16">
            <el-col :xs="24" :sm="12">
                <el-card class="mb16">
                    <p class="card-title">提交bug/功能请求</p>
                    <el-alert
                        title="如果出现了明显不合理的结果，请务必提交bug"
                        style="margin-bottom: 16px"
                        :closable="false"
                        type="warning"
                    ></el-alert>
                    <el-button @click="newPage(links.issue)" class="data-source">
                        github Issue
                        <font-awesome-icon :icon="['fab', 'github']"></font-awesome-icon>
                    </el-button>
                    <el-button class="data-source">
                        QQ(584130248)
                        <font-awesome-icon icon="comment"></font-awesome-icon>
                    </el-button>
                    <el-button class="data-source">
                        QQ群(801106595)
                        <font-awesome-icon icon="comment"></font-awesome-icon>
                    </el-button>
                </el-card>
            </el-col>
            <el-col :xs="24" :sm="12">
                <el-card class="mb16">
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
                <el-card class="mb16">
                    <p class="card-title">第一次使用？</p>
                    <el-button
                        @click="$router.push('/help/basic')"
                        class="data-source"
                    >
                        基本使用帮助
                        <i class="el-icon-question"></i>
                    </el-button>
                    <el-button
                        @click="$router.push('/help/export-tools')"
                        type="primary"
                        class="data-source"
                    >
                        导出工具
                        <i class="el-icon-aim"></i>
                    </el-button>
                    <el-button
                        @click="$router.push('/help/faq')"
                        class="data-source"
                    >
                        FAQ
                        <i class="el-icon-question"></i>
                    </el-button>
                </el-card>
            </el-col>
            <el-col :xs="24" :sm="12">
                <el-card class="mb16">
                    <p class="card-title">开发者文档</p>
                    <a class="el-button no-deco" :href="links.doc" target="_blank">
                        文档
                        <i class="el-icon-connection"></i>
                    </a>
                </el-card>
            </el-col>
        </el-row>

        <el-card style="margin-bottom: 16px">
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
import changelogs from "@/changelog/changelog";

import links from "@const/links";

export default {
    name: "IntroPage",
    created: function() {
        this.links = links;

        this.version = process.env.VERSION;
        this.webTitle = process.env.WEB_TITLE;

        this.lastestVersion = changelogs[changelogs.length - 1];
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

<style scoped>
.mb16 {
    margin-bottom: 16px;
}

.pay {
    display: flex;
    align-items: center;
    margin-top: 20px;
}

.pay span {
    width: 100px;
}

.pay img {
    width: 96px;
    height: 96px;
    display: block;
}

.update-banner {
    padding: 20px;
    color: #303133;
    background: rgba(225, 243, 216, 0.5);
    border: 1px solid #67c23a;
    margin-bottom: 16px;
    border-radius: 3px;
}

.title {
    font-size: 3rem;
    margin: 0;
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
</style>