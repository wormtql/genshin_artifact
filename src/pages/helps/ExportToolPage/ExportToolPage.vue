<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>帮助</el-breadcrumb-item>
            <el-breadcrumb-item>导出工具汇总</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-card class="item">
            <div slot="header">
                1. Genshin Art Scanner（推荐）
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

        <el-card class="item">
            <div slot="header">2. 椰羊（cocogoat）（推荐）</div>

            <p>使用tesseract进行本地OCR自动识别，基本实现全自动</p>

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

        <el-card class="item">
            <div slot="header">3. 蹦蹦炸弹（已暂停维护）</div>

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
export default {
    name: "ExportToolPage",
}
</script>

<style scoped>
.item {
    margin-bottom: 16px;
}
</style>