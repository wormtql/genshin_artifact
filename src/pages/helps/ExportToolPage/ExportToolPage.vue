<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>帮助</el-breadcrumb-item>
            <el-breadcrumb-item>导出工具汇总</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-card class="item" shadow="never">
            <div slot="header">
                <font-awesome-icon :icon="['fas', 'crown']"
                    style="color: #8a0045; margin-right: 4px;"
                ></font-awesome-icon>yas
            </div>

            <p>来自莫娜占卜铺原作者的导出器：Yet Another Scanner，state-of-the-art（速度上）导出器</p>

            <div class="footer">
                <a target="_blank" href="https://github.com/wormtql/yas"
                    class="el-button no-deco el-button--primary"
                >
                    <i class="el-icon-connection"></i>
                    项目地址
                </a>
            </div>
        </el-card>

        <el-card class="item" shadow="never">
            <div slot="header">
                <font-awesome-icon :icon="['fas', 'crown']"
                                   style="color: #8a0045; margin-right: 4px;"
                ></font-awesome-icon>「天目」-- Amenoma
            </div>

            <p>fork自Genshin Art Scanner</p>

            <div class="footer">
                <a target="_blank" href="https://github.com/daydreaming666/Amenoma"
                    class="el-button no-deco el-button--primary"
                >
                    <i class="el-icon-connection"></i>
                    项目地址
                </a>
            </div>
        </el-card>

        <el-card class="item archived" shadow="never">
            <div slot="header">
                Genshin Art Scanner
            </div>

            <p>本地识别，基本实现全自动，无UI模式，速度大致每分钟100-200个</p>

            <div class="footer">
                <a
                    target="_blank"
                    href="https://pan.baidu.com/s/1NSePEf3gzSA45aKASkfZVw"
                    class="el-button no-deco el-button--primary"
                >
                    <i class="el-icon-connection"></i>
                    百度网盘（提取码：sbqv）
                </a>
                <a target="_blank" href="https://github.com/ProblemFactory/GenshinArtScanner" class="el-button no-deco">
                    <i class="el-icon-connection"></i>
                    项目地址
                </a>
                <a
                    target="_blank"
                    href="https://bbs.nga.cn/read.php?tid=26425531&_ff=650&rand=246"
                    class="el-button no-deco"
                >
                    <i class="el-icon-connection"></i>
                    使用说明
                </a>
            </div>
        </el-card>

        <el-card class="item archived" shadow="never">
            <div slot="header">椰羊（cocogoat）</div>

            <p>本地识别，基本实现全自动，有UI</p>

            <div class="footer">
                <a target="_blank" href="https://77.cocogoat.work/v1/ascension/" class="el-button el-button--primary no-deco">
                    <i class="el-icon-connection"></i>
                    下载地址
                </a>
                <a target="_blank" href="https://github.com/YuehaiTeam/cocogoat" class="el-button no-deco">
                    <i class="el-icon-connection"></i>
                    项目地址
                </a>
            </div>
        </el-card>

        <el-card class="item archived" shadow="never">
            <div slot="header">蹦蹦炸弹</div>

            <p>使用百度API进行远程OCR自动识别，缺点是需要注册百度AI账号</p>

            <div class="footer">
                <a target="_blank" href="https://github.com/YuehaiTeam/cocogoat" class="el-button no-deco">
                    <i class="el-icon-connection"></i>
                    项目地址
                </a>
            </div>
        </el-card>
    </div>
</template>

<script>
let iter = 0;

export default {
    name: "ExportToolPage",
    methods: {
        getIndex() {
            iter += 1;
            return iter;
        }
    }
}
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