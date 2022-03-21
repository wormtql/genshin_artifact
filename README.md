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

## 贡献
### 添加目标函数
目标函数在`mona/src/target_functions/target_functions`
1. 在上述文件夹的对应位置建立新目标函数文件
2. 在`target_functions/target_function_name.rs`新建目标函数名
3. 创建一个struct，必须以`TargetFunction`结尾
```rust
pub struct NewTargetFunction {
    ...
}
```
4. 如果该函数有设置，在`target_functions/target_function_config.rs`新建同名enum
5. 为`NewTargetFunction`实现两个trait，`TargetFunctionMetaTrait`和`TargetFunction`
```rust
impl TargetFunctionMetaTrait for NewTargetFunction {
    // 该目标函数的元数据
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::GanyuDefault,
        chs: "chs",
        description: "description",
        tags: "tag1,tag2",
        four: TargetFunctionFor::SomeWho(CharacterName::Ganyu),
        image: TargetFunctionMetaImage::Avatar
    };

    // 目标函数的设置，没有设置可以省略
    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "melt_rate",
            title: "融化占比",
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        // create boxed target function
    }
}

impl TargetFunction for NewTargetFunction {
    // 可以参考其他文件
}
```
6. 在`target_functions/target_functions/<element>/mod.rs`中，重导出`NewTargetFunction`
```rust
// in <element>.rs
pub use new_target_function::NewTargetFunction;
```