<template>
    <div class="page-root">
        <p
            class="title"
        >
            <span class="mona">{{ webTitle }}</span>
            V{{ version }}
        </p>

        <el-row :gutter="16">
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item text="伤害计算器/单人配装" url="/calculate" :icon="IconFa6SolidCalculator"
                    description="- 通过计算机进行单人圣遗物自动搭配，以得到最高输出等内容。<br>- 响应式计算器，所见即所得。"
                ></use-case-item>
            </el-col>
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item text="队伍配装" url="/team-optimization" :icon="IconFa6SolidUserGroup"
                               description="队伍配装，圣遗物不冲突"
                ></use-case-item>
            </el-col>
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item text="圣遗物潜力" url="/potential" :icon="IconFa6SolidRuler"
                               description="快速计算圣遗物潜力。潜力即圣遗物满级后能够获得的平均得分，能够一定程度上衡量圣遗物的价值"
                ></use-case-item>
            </el-col>
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item text="圣遗物导出" url="/help/export-tools" :icon="IconFa6SolidFileExport"
                               description="快速导出圣遗物"
                ></use-case-item>
            </el-col>
        </el-row>

        <h2>开源地址</h2>
        <el-row :gutter="16">
            <el-col :xs="24" :sm="12" class="mb16">
                <use-case-item text="MONA" :icon="IconFa6BrandsGithub"
                               description="- 前端<br>- 莫娜DSL<br>- DSL Book等"
                               @click="newPage('https://github.com/wormtql/genshin_artifact')"
                ></use-case-item>
<!--                <div-->
<!--                    class="source-code-item"-->
<!--                    @click="newPage(links.frontendProject)"-->
<!--                >-->
<!--                    <el-icon :size="30"><i-fa6-brands-github></i-fa6-brands-github></el-icon>-->
<!--                    &lt;!&ndash;                        <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>&ndash;&gt;-->
<!--                    <p class="item-title">Mona</p>-->
<!--                </div>-->
            </el-col>
            <el-col :xs="24" :sm="12" class="mb16">
                <use-case-item text="Yas" :icon="IconFa6BrandsGithub"
                               description="- YAS导出器"
                               @click="newPage('https://github.com/wormtql/yas')"
                ></use-case-item>
<!--                <div-->
<!--                    class="source-code-item"-->
<!--                    @click="newPage(links.yasProject)"-->
<!--                >-->
<!--                    <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>-->
<!--                    <p class="item-title">YAS</p>-->
<!--                </div>-->
            </el-col>
        </el-row>

        <h2>反馈</h2>
        <el-row :gutter="16">
            <el-col :xs="24" :sm="8" class="mb16">
                <use-case-item text="Github Issue" :icon="IconFa6BrandsGithub"
                               description="在Github仓库提出Issue"
                               @click="newPage('https://github.com/wormtql/genshin_artifact/issues')"
                ></use-case-item>
            </el-col>
            <el-col :xs="24" :sm="8" class="mb16">
                <use-case-item text="QQ群" :icon="IconFa6BrandsQQ"
                               description="在QQ群反馈或是水群（群号：801106595）"
                               @click="newPage('https://qm.qq.com/cgi-bin/qm/qr?k=yQaJgPzRmBgEXXk1uiqNbq7CIrq-0biW&jump_from=webapi')"
                ></use-case-item>
            </el-col>
            <el-col :xs="24" :sm="8" class="mb16">
                <use-case-item text="NGA讨论贴" :icon="IconFa6SolidComment"
                               description="在NGA进行反馈"
                               @click="newPage('https://nga.178.com/read.php?tid=31180859')"
                ></use-case-item>
            </el-col>
        </el-row>

<!--        <el-row :gutter="16">-->
<!--            <el-col :xs="24" :sm="12">-->
<!--                <el-card class="mb16" shadow="never">-->
<!--                    <p class="card-title">圣遗物导出工具</p>-->
<!--                    <el-button-->
<!--                        @click="newPage(links.yasDownloadGithub)"-->
<!--                        type="primary"-->
<!--                        class="data-source"-->
<!--                    >-->
<!--                        YAS下载地址-->
<!--                        <i class="el-icon-aim"></i>-->
<!--                    </el-button>-->
<!--                    <el-button-->
<!--                        @click="$router.push('/help/export-tools')"-->
<!--                        class="data-source"-->
<!--                    >-->
<!--                        导出工具大全-->
<!--                        <i class="el-icon-aim"></i>-->
<!--                    </el-button>-->
<!--                </el-card>-->
<!--            </el-col>-->
<!--            <el-col :xs="24" :sm="12">-->
<!--                <el-card class="mb16" shadow="never">-->
<!--                    <p class="card-title">开发者文档</p>-->
<!--                    <a class="el-button no-deco" :href="links.doc" target="_blank">-->
<!--                        文档-->
<!--                        <i class="el-icon-connection"></i>-->
<!--                    </a>-->
<!--                </el-card>-->
<!--            </el-col>-->
<!--        </el-row>-->

<!--        <el-row :gutter="16">-->
<!--            <el-col :xs="24" :sm="12">-->
        <h2>请莫娜吃饭</h2>
                <el-card style="margin-bottom: 16px" shadow="never">
<!--                    <p class="card-title">请莫娜吃饭</p>-->
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
<!--            </el-col>-->
<!--            <el-col :xs="24" :sm="12">-->
<!--                <el-card shadow="never"></el-card>-->
<!--            </el-col>-->
<!--        </el-row>-->
    </div>
</template>

<script setup lang="ts">
import links from "@const/links"
import { createFeedback } from "@/api/misc"

import IconEpCpu from "~icons/ep/cpu"
import IconFa6SolidUserGroup from "~icons/fa6-solid/user-group"
import IconFa6SolidFileExport from "~icons/fa6-solid/file-export"
import IconFa6SolidCalculator from "~icons/fa6-solid/calculator"
import IconFa6SolidRuler from "~icons/fa6-solid/ruler"
import IconFa6SolidComment from "~icons/fa6-solid/comment"
import IconFa6BrandsGithub from "~icons/fa6-brands/github"
import IconFa6BrandsQQ from "~icons/fa6-brands/qq"

import UseCaseItem from "./UseCaseItem.vue"
import ButtonItem from "./ButtonItem.vue"
import {useRouter} from "vue-router"


const link = links
const version = process.env.MONA_VERSION
const webTitle = process.env.MONA_TITLE
const needMigrate = process.env.MONA_NEED_MIGRATE
const buildDate = process.env.MONA_BUILD_DATA
const host = location.hostname


const router = useRouter()

function navigateTo(r: any) {
    router.push(r)
}

function newPage(url: string) {
    window.open(url, "_blank")
}


const feedback = ref("")

function handleClickSubmitFeedback() {
    if (feedback.value === "") {
        return
    }
    createFeedback(feedback.value)

    ElMessage.success({
        message: "已发送"
    })
}
</script>

<style lang="scss" scoped>
.page-root {
    //height: 100%;
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