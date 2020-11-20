<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>主页</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <p class="title">原神圣遗物HUB V1.1.1</p>
        <!-- <p>Artifacts HUB</p> -->
        <p style="margin-bottom: 32px">
            <span style="color: #409EFF">圣遗物自动配装</span> / 
            <span style="color: #67C23A">角色养成方向推荐</span> / 
            <span style="color: #E6A23C">面板计算</span>
        </p>

        <!-- <el-alert title="注意：所有数值不用换算，均照游戏中的"></el-alert> -->

        <!-- <h3>介绍</h3>
        <ul>
            <li>计算最佳圣遗物搭配，根据不同的目标函数有不同的搭配。</li>
            <li>面板计算</li>
        </ul> -->
        


        <!-- <h3>使用方法</h3>
        <el-steps :active="100">
            <el-step title="写入圣遗物" icon="el-icon-s-help"></el-step>
            <el-step title="配置基础属性" description="包括角色、武器"></el-step>
            <el-step title="配置目标函数" description="不同的养成计划有不同的目标函数 "></el-step>
        </el-steps> -->

        <el-card style="margin-bottom: 16px">
            <p style="margin: 0; padding: 0 0 16px 0; font-size: 1.5rem;">帮助</p>
            <el-row :gutter="16">
                <el-col :span="12">
                    <el-card :body-style="{textAlign: 'center'}" @click.native="navigateTo('/use-case')" class="item"
                        shadow="hover"
                    >
                        <font-awesome-icon class="icon" icon="question-circle"></font-awesome-icon>
                        <p class="item-title">使用示例</p>
                    </el-card>
                </el-col>
                <el-col :span="12">
                    <el-card :body-style="{textAlign: 'center'}" @click.native="navigateTo('/alg')" class="item"
                        shadow="hover"
                    >
                        <font-awesome-icon icon="percent" class="icon"></font-awesome-icon>
                        <p class="item-title">算法说明</p>
                    </el-card>
                </el-col>
            </el-row>
        </el-card>

        <!-- <h3>梯度</h3>
        <p>最终期望输出其实是攻击力、暴击率、暴击伤害等的函数，对这个函数的变量求偏导，可以求得往哪个方向发展会有更大的期望输出。</p>
        <p>例如，暴击率的偏导大于暴击伤害的偏导，则同样提升1%的情况下，暴击率会有更大的提升</p>
        <p>需要注意的是，并不是往偏导最大的方向发展是最优的，应该往“梯度”方向发展。例如，暴击率偏导是2，暴击伤害偏导是3，则暴击+4%，暴击伤害+6%比单纯暴击伤害+10%效果更好</p> -->
        
        <!-- <h3>自定义</h3>
        <p>由于数据不完善，只支持少部分等级的武器和角色，因此可以自定义</p> -->

        <!-- <h3>注意</h3>
        <p>由于是枚举的算法，因此数据量不能太大，添加圣遗物的时候应当把明显更次的圣遗物去掉或者禁用</p>
        <p>输入数据时，所有百分比数值应当输入小数，例如，攻击力+3.7%应该输入为0.037</p> -->

        <el-card style="margin-bottom: 16px">
            <p style="margin: 0; padding: 0 0 16px 0; font-size: 1.5rem;">github地址</p>
            <el-row :gutter="16">
                <el-col :span="12">
                    <el-card :body-style="{textAlign: 'center'}" @click.native="newPage(frontendUrl)" class="item"
                        shadow="hover"
                    >
                        <font-awesome-icon :icon="['fab', 'github']" class="icon"></font-awesome-icon>
                        <p class="item-title">前端</p>
                    </el-card>
                </el-col>
                <el-col :span="12">
                    <el-card :body-style="{textAlign: 'center'}" @click.native="newPage(panelUrl)" class="item"
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
                    <p style="font-size: 1.2rem; margin: 0 0 16px 0">联系我们</p>
                    <el-button @click="newPage(issue)">
                        github Issue
                        <font-awesome-icon :icon="['fab', 'github']"></font-awesome-icon>
                    </el-button>
                    <el-button @click="newPage(nga)">
                        NGA
                        <font-awesome-icon icon="comment"></font-awesome-icon>
                    </el-button>
                </el-card>
                <el-card style="margin-bottom: 16px">
                    <p style="font-size: 1.2rem; margin: 0 0 16px 0">参与开发</p>
                    <p style="margin: 0">急需数据整理、角色培养目标函数</p>
                    <p style="margin: 0">qq群：801106595</p>
                </el-card>
                <el-card>
                    <p style="font-size: 1.2rem; margin: 0 0 16px 0">数据来源</p>
                    <el-button @click="newPage(wiki)">
                        bilibili原神wiki
                        <font-awesome-icon icon="database"></font-awesome-icon>
                    </el-button>
                </el-card>
            </el-col>
            <el-col :span="12">
                <el-card>
                    <p style="font-size: 1.2rem; margin: 0 0 16px 0">捐助</p>
                    <el-row :gutter="16">
                        <el-col :span="12">
                            <el-image :src="wechat"></el-image>
                        </el-col>
                        <el-col :span="12">
                            <el-image :src="alipay"></el-image>
                        </el-col>
                    </el-row>
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
import wechat from "./wechat.jpg";
import alipay from "./alipay.jpg";

export default {
    name: "IntroPage",
    data: function() {
        return {
            frontendUrl: "https://github.com/wormtql/genshin_artifact",
            panelUrl: "https://github.com/wormtql/genshin_panel",
            issue: "https://github.com/wormtql/genshin_artifact/issues",
            nga: "https://bbs.nga.cn/read.php?tid=24184608",
            wiki: "https://wiki.biligame.com/ys/%E9%A6%96%E9%A1%B5",

            wechat,
            alipay,
        }
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
</style>