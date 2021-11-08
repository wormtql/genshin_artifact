# 贡献目标函数指南

## 文件
在`src/assets/target_functions/data`下建立你要写的函数文件夹，并创建至少两个文件：  
- `xxx.tf.func.js`：计算的函数
- `xxx.tf.js`：meta信息


可以创建一个配置组件:
- `xxx.tcfg.vue`

## `xxx.tf.js`
需要default导出一个Object：
```js
import badge from "@asset/badges/sword.png";
import config from "./AboveCrit.tcfg.vue";

export default {
    name: "aboveCrit",    // 唯一的标识
    chs: "固定暴击率",     // 显示的中文
    description: [        // 描述
        "优先堆暴击率到给定阈值，再堆攻击和爆伤",
    ],
    tags: [               // 标签
        "输出",
    ],
    "for": "common",      // 如果是角色专属，就写角色名，公共的写common
    badge,                // 显示的图标
    config,               // 配置目标函数的vue组件（可选）
}
```

## `xxx.tf.func.js`
需要default导出一个Object：
```js
export default {
    name: "aboveCrit",    // 与上面保持一致
    func: f,              // 用于计算的函数
    needContext: false,   // 是否需要圣遗物上下文，如果需要使用计算中的圣遗物信息，应当设为true，但会降低运行效率
    version: 2,           // 固定
}
```

### 计算函数的定义
例如：
```js
// attribute: 面板
// cArgs: 角色数据
// wArgs: 武器数据
// tArgs: 目标函数数据
// context: 圣遗物上下文
// return: 一个数，越大代表配装越优
function f(attribute, { cArgs, wArgs, tArgs }, context) {
    

    return dps;
}
```

### `cArgs`
```typescript
// 角色参数
interface CARGS {
    level: number,
    ascend: boolean,
    skill1: number,
    skill2: number,
    skill3: number,
    constellation: number,
    name: string,
    args: any,
}
```

### `wArgs`
```typescript
// 武器参数
interface WARGS {
    name: string,
    level: number,
    ascend: boolean,
    refine: number,
    args: any,
}
```

### `tArgs`
```typescript
// 目标函数参数
interface TARGS {
    [key: string]: any
}
```

### `context`
```typescript
interface CONTEXT {
    artifactSet: {
        [key: string]: number, // 圣遗物名字，及其数量
    }
}
```

### `attribute`
见 [attribute.ts](https://github.com/wormtql/genshin_panel/blob/main/src/attribute/attribute.ts)

## `xxx.tcfg.vue`
vue组件需要实现两个方法
```vue
<template>
    ...
</template>

<script>
export default {
    name: "AboveCrit.tcfg",
    data() {
        return {
            threshold: "0.5",
        }
    },
    methods: {
        compact() {         // 获取tArgs
            return {
                threshold: parseFloat(this.threshold) ?? 0.5,
            }
        },
        setData(d) {        // 设置tArgs
            this.threshold = d.threshold.toString();
        }
    }
    
}
</script>
```

## 其他
### 获取角色技能
可以查看`src/assets/characters/`下的`xxx.skill.js`文件
```js
import { charactersData } from "@asset/characters";

const skill = charactersData["dadaliya"].skill;
```