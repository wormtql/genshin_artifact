# 内置函数
无需导入，可以直接使用

## print
打印一个或多个值
```
print(1)
print(1, 2, 3)
```

## type
获取值的类型
```
dmg a = Amber.Normal1
print(type(a))
print(type(1))
print(type(true))
```

## max/min
获取一组数的最大/最小值，每个值都必须为number类型，否则报错
```
m = min(1, 2, 3)
M = max(1, 2, 3)
print(m, M)
```

```
// Error: TypeError, expecting `number`, found `bool`
m = min(1, true)
```