# genshin_artifacts
[网站](http://39.107.68.17:5000/#/calculate)

  
## 简介
《原神》圣遗物自动搭配
《原神》面板计算（包括圣遗物套装效果等）

## 目标函数
普通的输出型目标函数由四部分组成  
+ 普攻：频率、倍率、元素伤害占比
+ 重击：频率、倍率、元素伤害占比
+ 元素战技（E）：频率、倍率
+ 元素爆发（Q）：频率、倍率  


由以上四部分的期望作为输出型目标函数  
其他期望函数包括单个属性等，可以求得单个属性的最大化  
一些角色有自己的期望函数，例如七七等

## Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

### Lints and fixes files
```
npm run lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
