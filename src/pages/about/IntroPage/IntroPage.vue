<template>
    <div class="page-root">
        <p
            class="title"
        >
            <span class="mona">{{ webTitle }}</span>
            V{{ version }}
        </p>

        <el-button @click="handleTest"></el-button>

        <el-row :gutter="16">
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item :text="t('intro.useCase1')" url="/calculate" :icon="IconFa6SolidCalculator"
                    :description="t('intro.useCase1Description')"
                ></use-case-item>
            </el-col>
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item :text="t('intro.useCase2')" url="/team-optimization" :icon="IconFa6SolidUserGroup"
                               :description="t('intro.useCase2Description')"
                ></use-case-item>
            </el-col>
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item :text="t('intro.useCase3')" url="/potential" :icon="IconFa6SolidRuler"
                               :description="t('intro.useCase3Description')"
                ></use-case-item>
            </el-col>
            <el-col :sm="6" :xs="24" class="mb16">
                <use-case-item :text="t('intro.useCase4')" url="/help/export-tools" :icon="IconFa6SolidFileExport"
                               :description="t('intro.useCase4Description')"
                ></use-case-item>
            </el-col>
        </el-row>

        <h2>{{ t("intro.opensource") }}</h2>
        <el-row :gutter="16">
            <el-col :xs="24" :sm="12" class="mb16">
                <use-case-item text="MONA" :icon="IconFa6BrandsGithub"
                               :description="t('intro.opensourceMonaDescription')"
                               @click="newPage('https://github.com/wormtql/genshin_artifact')"
                ></use-case-item>

            </el-col>
            <el-col :xs="24" :sm="12" class="mb16">
                <use-case-item text="Yas" :icon="IconFa6BrandsGithub"
                               :description="t('intro.opensourceYasDescription')"
                               @click="newPage('https://github.com/wormtql/yas')"
                ></use-case-item>
            </el-col>
        </el-row>

        <h2>{{ t("intro.feedback") }}</h2>
        <el-row :gutter="16">
            <el-col :xs="24" :sm="8" class="mb16">
                <use-case-item :text="t('intro.fbGithubIssue')" :icon="IconFa6BrandsGithub"
                               :description="t('intro.fbIssueDescription')"
                               @click="newPage('https://github.com/wormtql/genshin_artifact/issues')"
                ></use-case-item>
            </el-col>
            <el-col :xs="24" :sm="8" class="mb16">
                <use-case-item :text="t('intro.fbQQ')" :icon="IconFa6BrandsQQ"
                               :description="t('intro.fbQQDescription')"
                               @click="newPage('https://qm.qq.com/cgi-bin/qm/qr?k=yQaJgPzRmBgEXXk1uiqNbq7CIrq-0biW&jump_from=webapi')"
                ></use-case-item>
            </el-col>
            <el-col :xs="24" :sm="8" class="mb16">
                <use-case-item :text="t('intro.fbNGA')" :icon="IconFa6SolidComment"
                               :description="t('intro.fbNGADescription')"
                               @click="newPage('https://nga.178.com/read.php?tid=31180859')"
                ></use-case-item>
            </el-col>
        </el-row>

        <h2>请莫娜吃饭</h2>
        <el-card style="margin-bottom: 16px" shadow="never">
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

<script setup lang="ts">
import { createFeedback } from "@/api/misc"

import IconFa6SolidUserGroup from "~icons/fa6-solid/user-group"
import IconFa6SolidFileExport from "~icons/fa6-solid/file-export"
import IconFa6SolidCalculator from "~icons/fa6-solid/calculator"
import IconFa6SolidRuler from "~icons/fa6-solid/ruler"
import IconFa6SolidComment from "~icons/fa6-solid/comment"
import IconFa6BrandsGithub from "~icons/fa6-brands/github"
import IconFa6BrandsQQ from "~icons/fa6-brands/qq"

import UseCaseItem from "./UseCaseItem.vue"
import {useRouter} from "vue-router"
import {useI18n} from "@/i18n/i18n"


const version = process.env.MONA_VERSION
const webTitle = process.env.MONA_TITLE
const needMigrate = process.env.MONA_NEED_MIGRATE
const buildDate = process.env.MONA_BUILD_DATA
const host = location.hostname


const router = useRouter()

const { t } = useI18n()

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

async function handleTest() {
    const { setLocale } = useI18n()
    await setLocale("en")
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