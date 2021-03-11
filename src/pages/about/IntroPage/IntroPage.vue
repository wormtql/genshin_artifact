<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>主页</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <p class="title"><span class="mona">莫娜占卜铺</span> V{{ version }} Welcome</p>
        <p style="padding: 0; margin: 0; font-size: 10px;">该网站处处透露着贫穷，欢迎有志者参与开发（美术等）</p>
        <p style="margin-bottom: 32px">
            <span style="color: #409EFF">圣遗物自动配装/圣遗物分数与潜力值</span>
            <!-- <span style="color: #67C23A">角色养成方向推荐</span> / 
            <span style="color: #E6A23C">面板计算</span> -->
        </p>

        <div class="update-banner">
            <p style="font-size: 24px; margin: 0;">V{{ version }}更新内容</p>
            <ul>
                <li
                    v-for="(item, index) in lastestVersion.changes"
                    :key="index"
                >{{ item }}</li>
            </ul>
        </div>

        <el-card style="margin-bottom: 16px">
            <p class="big-card-title">功能</p>
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

        <el-card style="margin-bottom: 16px">
            <p class="big-card-title">github地址（求star(/doge)）</p>
            <el-alert
                title="该工具全部免费且开源"
                :closable="false"
                style="margin-bottom: 16px"
            ></el-alert>
            <el-row :gutter="16">
                <el-col :span="12">
                    <el-card
                        :body-style="{textAlign: 'center'}"
                        @click.native="newPage(urls.frontendUrl)" class="item"
                        shadow="hover"
                    >
                        <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>
                        <p class="item-title">前端</p>
                    </el-card>
                </el-col>
                <el-col :span="12">
                    <el-card
                        :body-style="{textAlign: 'center'}"
                        @click.native="newPage(urls.panelUrl)" class="item"
                        shadow="hover"
                    >
                        <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>
                        <p class="item-title">面板计算器（npm包）</p>
                    </el-card>
                </el-col>
            </el-row>
        </el-card>

        <el-row :gutter="16">
            <el-col :span="12">
                <el-card style="margin-bottom: 16px">
                    <p class="card-title">提交bug</p>
                    <el-alert
                        title="如果出现了明显不合理的结果，请务必提交bug"
                        style="margin-bottom: 16px"
                        :closable="false"
                        type="warning"
                    ></el-alert>
                    <el-button @click="newPage(urls.issue)">
                        github Issue
                        <font-awesome-icon :icon="['fab', 'github']"></font-awesome-icon>
                    </el-button>
                    <el-button>
                        QQ(584130248)
                        <font-awesome-icon icon="comment"></font-awesome-icon>
                    </el-button>
                    <el-button>
                        QQ群(801106595)
                        <font-awesome-icon icon="comment"></font-awesome-icon>
                    </el-button>
                </el-card>

                <el-card style="margin-bottom: 16px">
                    <p class="card-title">请莫娜吃饭</p>
                    <el-alert
                        :closable="false"
                        title="莫娜多可怜，各位老板若觉得工具好用，可以请莫娜吃饭，莫娜会帮我支付服务器费用（手动狗头）"
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
            </el-col>
            <el-col :span="12">
                <el-card>
                    <p class="card-title">数据来源</p>
                    <el-button @click="newPage(urls.wiki)" class="data-source">
                        bilibili原神wiki
                        <font-awesome-icon icon="database"></font-awesome-icon>
                    </el-button>
                    <el-button @click="newPage(urls.wiki2)" class="data-source">
                        Genshin Impact Wiki | Fandom
                        <font-awesome-icon icon="database"></font-awesome-icon>
                    </el-button>
                    <el-button @click="newPage(urls.wiki3)" class="data-source">
                        旅行者创作平台-观测枢-原神wiki
                        <font-awesome-icon icon="database"></font-awesome-icon>
                    </el-button>
                </el-card>
            </el-col>
        </el-row>

        <!-- <h3>数据</h3>
        <p>网站使用的数据来源：</p>
        <ul>
            <li><a href="https://wiki.biligame.com/ys/%E9%A6%96%E9%A1%B5">bilibili原神wiki</a></li>
        </ul> -->
    </div>
</template>

<script>
import changelogs from "@/changelog/changelog";

export default {
    name: "IntroPage",
    created: function() {
        this.urls = {
            frontendUrl: "https://github.com/wormtql/genshin_artifact",
            panelUrl: "https://github.com/wormtql/genshin_panel",
            issue: "https://github.com/wormtql/genshin_artifact/issues",
            nga: "https://bbs.nga.cn/read.php?tid=24184608",
            wiki: "https://wiki.biligame.com/ys/%E9%A6%96%E9%A1%B5",
            wiki2: "https://genshin-impact.fandom.com/wiki/Genshin_Impact_Wiki",
            wiki3: "https://bbs.mihoyo.com/ys/obc/?bbs_presentation_style=no_header",
        };

        this.version = process.env.VERSION;

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
    margin-right: 16px;
    margin-top: 16px;
}

.mona {
    background: rgb(86,72,132);
    border-radius: 3px;
    color: white;
    padding: 0 8px;
}
</style>