<div align="center">


# 莫娜占卜铺
[![Netlify Status](https://api.netlify.com/api/v1/badges/3a2fc38c-d9dd-4257-98d5-11891cf9b064/deploy-status)](https://app.netlify.com/sites/jovial-chandrasekhar-293ccd/deploys)
![GitHub Repo stars](https://img.shields.io/github/stars/wormtql/genshin_artifact)
![GitHub forks](https://img.shields.io/github/forks/wormtql/genshin_artifact)

QQ群：1035783390
</div>


## 简介
### 伤害计算与分析
- 增删BUFF
- 参数调整
- 伤害明细
- 面板构成
### 圣遗物配装与分析
- 虚拟圣遗物优化算法
- 启发式优化算法
- 队伍圣遗物自动搭配
- 词条收益曲线
- 圣遗物养成推荐
- 圣遗物词条分析
- 圣遗物潜力与评分
### 数据库
- 基于计算结果的圣遗物、武器统计

## 本地运行
### 环境
该项目依赖[Rust](https://www.rust-lang.org/) 
1. 安装Rust工具链，详见官网
2. 安装Rust Webassembly工具链（wasm-pack）
3. node
### 运行步骤
1. 克隆仓库
```
git clone --recursive https://github.com/wormtql/genshin_artifact
```
2. 编译wasm依赖
```
npm run build:wasm
```
3. 生成数据文件（武器、角色、圣遗物等的信息）
```
npm run gen_meta
```
4. 安装 npm 依赖
```
npm install
```
5. 运行
```
npm run serve
```
6. 打包
```
npm run build
```

[//]: # (## Docker)

[//]: # (```)

[//]: # (docker build -t mona .)

[//]: # (docker run -dp 8080:80 mona)

[//]: # (```)

[//]: # (## 贡献)

[//]: # (### 添加目标函数)

[//]: # (目标函数位于[https://github.com/wormtql/mona-core]&#40;mona-core&#41;  )

[//]: # (`src/target_functions/target_functions`)

[//]: # (1. 在上述文件夹的对应位置建立新目标函数文件)

[//]: # (2. 在`target_functions/target_function_name.rs`新建目标函数名)

[//]: # (3. 创建一个struct，必须以`TargetFunction`结尾)

[//]: # (```rust)

[//]: # (pub struct NewTargetFunction {)

[//]: # (    ...)

[//]: # (})

[//]: # (```)

[//]: # (4. 如果该函数有设置，在`target_functions/target_function_config.rs`新建同名enum)

[//]: # (5. 为`NewTargetFunction`实现两个trait，`TargetFunctionMetaTrait`和`TargetFunction`)

[//]: # (```rust)

[//]: # (impl TargetFunctionMetaTrait for NewTargetFunction {)

[//]: # (    // 该目标函数的元数据)

[//]: # (    #[cfg&#40;not&#40;target_family = "wasm"&#41;&#41;])

[//]: # (    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {)

[//]: # (        name: TargetFunctionName::GanyuDefault,)

[//]: # (        chs: "chs",)

[//]: # (        description: "description",)

[//]: # (        tags: "tag1,tag2",)

[//]: # (        four: TargetFunctionFor::SomeWho&#40;CharacterName::Ganyu&#41;,)

[//]: # (        image: TargetFunctionMetaImage::Avatar)

[//]: # (    };)

[//]: # ()
[//]: # (    // 目标函数的设置，没有设置可以省略)

[//]: # (    #[cfg&#40;not&#40;target_family = "wasm"&#41;&#41;])

[//]: # (    const CONFIG: Option<&'static [ItemConfig]> = Some&#40;&[)

[//]: # (        ItemConfig {)

[//]: # (            name: "melt_rate",)

[//]: # (            title: "融化占比",)

[//]: # (            config: ItemConfig::RATE01_TYPE)

[//]: # (        })

[//]: # (    ]&#41;;)

[//]: # ()
[//]: # (    fn create&#40;character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig&#41; -> Box<dyn TargetFunction> {)

[//]: # (        // create boxed target function)

[//]: # (    })

[//]: # (})

[//]: # ()
[//]: # (impl TargetFunction for NewTargetFunction {)

[//]: # (    // 可以参考其他文件)

[//]: # (})

[//]: # (```)

[//]: # (6. 在`target_functions/target_functions/<element>/mod.rs`中，重导出`NewTargetFunction`)

[//]: # (```rust)

[//]: # (// in <element>.rs)

[//]: # (pub use new_target_function::NewTargetFunction;)

[//]: # (```)
