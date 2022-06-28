<template>
    <div>
        <export-tool-item
            v-for="(item, index) in items"
            :key="index"
            :name="item.name"
            :description="item.description"
            :buttons="item.buttons"
            class="item"
        ></export-tool-item>

        <el-collapse>
            <el-collapse-item title="其他工具">
                <el-alert title="这些工具已经停更，请勿使用" type="warning" :closable="false" style="margin-bottom: 12px"></el-alert>
                <export-tool-item
                    v-for="(item, index) in nonActiveItems"
                    :key="index"
                    :name="item.name"
                    :description="item.description"
                    :buttons="item.buttons"
                    class="item"
                ></export-tool-item>
            </el-collapse-item>
        </el-collapse>
    </div>
</template>

<script setup lang="ts">
import ExportToolItem from "./ExportToolItem.vue"

import IconFa6BrandsGithub from "~icons/fa6-brands/github"

const items = [
    {
        name: "yas",
        description: "来自莫娜占卜铺原作者的导出器：Yet Another Scanner，state-of-the-art（速度上）导出器",
        buttons: [
            {
                label: "项目地址",
                primary: true,
                link: "https://github.com/wormtql/yas",
                icon: IconFa6BrandsGithub
            }
        ]
    },
    {
        name: "「天目」Amenoma",
        description: "fork自Genshin Art Scanner",
        buttons: [
            {
                label: "项目地址",
                primary: true,
                link: "https://github.com/daydreaming666/Amenoma",
                icon: IconFa6BrandsGithub
            }
        ]
    }
]

const nonActiveItems = [
    {
        name: "Genshin Art Scanner（已停更）",
        description: "本地识别，基本实现全自动，无UI模式，速度大致每分钟100-200个",
        buttons: [
            {
                label: "项目地址",
                primary: true,
                link: "https://github.com/ProblemFactory/GenshinArtScanner",
                icon: IconFa6BrandsGithub
            }
        ]
    },
    {
        name: "椰羊cocogoat（已停更）",
        description: "本地识别，基本实现全自动，有UI。椰羊的原项目地址已经改为椰羊的成就管理",
        buttons: [
            {
                label: "项目地址",
                primary: true,
                link: "https://github.com/YuehaiTeam/cocogoat",
                icon: IconFa6BrandsGithub
            },
            {
                label: "下载地址",
                primary: false,
                link: "https://77.cocogoat.work/v1/ascension/",
                icon: IconFa6BrandsGithub
            }
        ]
    },
    {
        name: "蹦蹦炸弹（已停更）",
        description: "使用百度API进行远程OCR自动识别，缺点是需要注册百度AI账号",
        buttons: [
            {
                label: "项目地址",
                primary: true,
                link: "https://github.com/kites262/JumpyDumpty",
                icon: IconFa6BrandsGithub
            },
        ]
    },
]

// export default {
//     name: "ExportToolPage",
//     methods: {
//         getIndex() {
//             iter += 1;
//             return iter;
//         }
//     }
// }
</script>

<style scoped>
.item {
    margin-bottom: 16px;
}

.archived {
    opacity: 0.5;
    /*background-image: linear-gradient(45deg, #B2A2CD 25%, #5F5370 25%, #5F5370 50%, #B2A2CD 50%, #B2A2CD 75%, #5F5370 75%, #5F5370 100%);*/
    /*background-size: 56.57px 56.57px;*/
}
</style>