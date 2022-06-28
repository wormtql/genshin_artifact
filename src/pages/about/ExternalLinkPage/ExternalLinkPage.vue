<template>
    <div class="container">
        <external-link-item
            v-for="(item, index) in items"
            :item="item"
        ></external-link-item>
    </div>
</template>

<script setup lang="ts">
import ExternalLinkItem from "./ExternalLinkItem.vue"

import IconEpConnection from "~icons/ep/connection"
import IconEpChatRound from "~icons/ep/chat-round"
import type {LinkItem} from "./types"

const items: LinkItem[] = [
    {
        name: "可莉特调",
        tags: [
            "圣遗物评分",
            "角色/武器计算",
            "素材表",
        ],
        url: [
            { title: "网址", link: "https://genshin.pub", icon: IconEpConnection, primary: false }
        ]
    },
    {
        name: "原神地图",
        tags: [
            "地图工具"
        ],
        url: [
            { title: "网址", link: "https://yuanshen.site/", icon: IconEpConnection, primary: false }
        ]
    },
    {
        name: "MEW原神社区",
        tags: [
            "社区"
        ],
        url: [
            { title: "网址", link: "https://mew.fun/n/ys", icon: IconEpConnection, primary: false }
        ]
    },
    {
        name: "NOKNOK",
        tags: [
            "社区"
        ],
        description: "我们也是一个纯公益性质的游戏社区，希望给原神的小伙伴们一个更方便便捷的沟通阵地，可以更快的交流心得和组队",
        url: [
            { title: "网址", link: "https://www.noknok.cn/", icon: IconEpConnection, primary: false },
            { title: "原神群组", link: "https://www.noknok.cn/act/share_group_20210625/index.html?uid=100000041&gid=117757", icon: IconEpConnection, primary: false }
        ]
    },
    {
        name: "兴趣来源",
        tags: [
            "游戏助手", "工具",
        ],
        url: [
            { title: "网址", link: "https://xingqulaiyuan.chenyuwulu.top/", icon: IconEpConnection, primary: false }
        ]
    },
    {
        name: "某大佬的社区",
        tags: [
            "社区", "语音", "信息聚合"
        ],
        description: "这里是由热爱原神的旅行者们所打造的原神社区，在这里你可以寻求大佬的帮助，快速的度过萌新阶段，进一步了解原神的更多有趣好玩的活动、故事，也以在这里扩展自己对原神的认知！欢迎你的加入，旅行者，愿风神护佑你。",
        url: [
            { title: "开黑啦语音社区", link: "https://kaihei.co/QQ6zQ4", primary: false, icon: IconEpChatRound },
            { title: "MEW文字社区", link: "https://mew.fun/n/mimi", primary: false, icon: IconEpConnection }
        ]
    }
]

</script>

<style scoped lang="scss">

.container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    //grid-template-columns: repeat(auto-fill, minmax(350px, 400px));
    gap: 8px;
}

</style>