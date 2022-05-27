# Overview

MONA-DSL是一个用于自定义输出模式的语言。  
在圣遗物自动配装中，已有的预设可能并不能满足许多需求，该语言则通过代码的方式，编写自定义的目标函数，以满足更加多样化的需求  
同时，目标函数与优化引擎解耦，更加灵活

> **warning** 该语言正在积极开发中，各项功能尚未稳定，bug较多

## Hello World
以绫华为例，我们需要自定义其大招期望输出融化和普通各占一半，且伤害发生在冲刺之后（天赋18%冰伤加成）
```
dmg q = KamisatoAyaka.Q1({ after_dash: true })
normal = q.normal.e
melt = q.melt.e
result = normal + melt
```
使用一个名称result作为最后输出，优化引擎能够找到这个值，并得到其中的数值。如果result不是一个数值，则会出错
