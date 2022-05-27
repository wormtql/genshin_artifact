# 冰系角色

## 绫华-仅大招
```
dmg q = KamisatoAyaka.Q1
result = q.normal.e
```

## 绫华-大招、重击、普攻按比例
```
dmg q = KamisatoAyaka.Q1
dmg a = KamisatoAyaka.Normal1
dmg b = KamisatoAyaka.Charged

result = q.normal.e * 0.5 + a.normal.e * 10 + b.normal.e * 2
```

## 甘雨-纯重击
```
dmg b = Ganyu.Charged3
result = b.normal.e
```