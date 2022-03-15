<div align="center">


# 莫娜占卜铺
[![Netlify Status](https://api.netlify.com/api/v1/badges/3a2fc38c-d9dd-4257-98d5-11891cf9b064/deploy-status)](https://app.netlify.com/sites/jovial-chandrasekhar-293ccd/deploys)
![GitHub Repo stars](https://img.shields.io/github/stars/wormtql/genshin_artifact)
![GitHub forks](https://img.shields.io/github/forks/wormtql/genshin_artifact)


</div>

[//]: # (原神圣遗物分析工具，[网址在这里]&#40;https://www.mona-uranai.com&#41;  )
[//]: # (该README有以下语言：  )
[//]: # ([English]&#40;./README_en.md&#41;)
## 简介
- 圣遗物自动搭配
- 队伍圣遗物自动搭配
- 响应式伤害计算器
- 圣遗物潜力
- 圣遗物养成推荐
- more...

## 本地运行
该项目依赖[Rust](https://www.rust-lang.org/) 
1. 安装Rust工具链，详见官网
2. 安装Rust Webassembly工具链（wasm-pack）
3. 安装node依赖
```
npm install
```
4. 编译Rust依赖
```
cd mona
cargo build --release
wasm-pack build
```
5. 本地运行
```
npm run serve
```
