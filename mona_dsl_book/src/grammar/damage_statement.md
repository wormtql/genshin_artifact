# 伤害声明
在圣遗物配装优化过程中，目标函数需要反复执行。由于伤害计算是一个计算量相对较大的操作（相对于加减乘除等基本运算），因此，最好尽可能减少伤害声明的数量

伤害声明表示在角色上下文环境中获取角色某个技能的伤害，包括变量名、角色名、技能名、技能设置等。

## 普通伤害
### NormalDamage
```
dmg a = KamisatoAyaka.Normal1({ after_dash: true })
```
- `a`即变量名，将会加入到当前作用域，以便后来的操作进行访问。
- `KamisatoAyaka`为角色名，表示绫华
- `Normal1`为技能名，表示绫华的普攻第一段
- `{ after_dash: true }`为技能配置，表示该技能发生在冲刺之后

声明的普通伤害是一个`NormalDamage`类型的值，其包括普通元素/物理伤害、融化/蒸发伤害等
```
dmg a = KamisatoAyaka.Normal1
print(type(a))
```
输出
```
normal_damage
```

### DamageNumber
`NormalDamage`类型包括普通元素/物理伤害、融化/蒸发伤害等，其中每种伤害又是一个`DamageNumber`类型的值

- `a.normal`普通伤害
- `a.melt`融化伤害
- `a.vaporize`蒸发伤害
```
dmg a = Amber.Normal1
print(type(a.normal))
```
输出
```
damage_number
```

`DamageNumber`包括期望伤害、暴击伤害、非暴击伤害
- `e`期望伤害
- `c`暴击伤害
- `n`非暴击伤害
```
dmg a = Amber.Normal1
normal = a.normal
print(normal.e, normal.c, normal.n)
```
输出（具体数值随环境而异）
```
368.31072106709996
464.06222729999996
309.37481819999994
```

### 增幅反应
并不是所有技能都有蒸发、融化伤害，如果某个技能并不带有某个伤害，则会报错
```
dmg a = KamisatoAyaka.Normal1({ after_dash: true })
// ok
print(a.melt.e)
```
输出
```
884.4099021172799
```

反例
```
dmg a = KamisatoAyaka.Normal1
// error
print(a.melt.e)
```
输出
```
Runtime Error: DamageNotFound, damage `melt` not exist
```

## 治疗量
治疗量的声明与普通伤害相同，其没有融化和蒸发伤害，且其暴击、非暴击、期望伤害相等
```
dmg a = SangonomiyaKokomi.QHeal1
print(a.normal)
```

## 剧变反应
包括四种元素的扩散、感电、超载、超导、碎冰

其声明需要把普通伤害声明中的技能名改为`transformative`，且没有技能设置
```
dmg a = Amber.transformative
print(type(a))
```
输出
```
transformative_damage
```

## TransformativeDamage
声明之后的变量为`TransformativeDamage`类型

- `swirl_cryo`冰扩散
- `swirl_pyro`火扩散
- `swirl_electro`雷扩散
- `swirl_hydro`水扩散
- `overload`超载
- `electro_charged`感电
- `super_conduct`超导
- `shatter`碎冰

```
dmg a = Amber.transformative
print(a.overload)
```
输出
```
2604.6
```