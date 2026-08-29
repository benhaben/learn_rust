"""必考：.loc 一次写进原表；链式 df[mask][col]= 可能改在副本上

运行：python 02_pandas_loc.py

# 直接赋值和 loc 差在哪

df["side"] = 1
    整列换成 1。简单、改的是原 DataFrame 的这一列。
    不能顺便说「只改 ret>0 的那些行」。

df.loc[df["ret"] > 0, "side"] = 1
    同一句话里指定：哪些行 + 哪一列 + 写成什么。
    pandas 在这一次 __setitem__ 里改原表，不会先切出一张可能是副本的表。

df[df["ret"] > 0]["side"] = 1
    两步，中间那张表是「原表的一块」还是「新拷出来的表」，pandas 不保证。

    想成 C++：
      view  = span / 引用，写它 = 写原内存
      copy  = 新数组，写它 = 原数组不动
    df[mask] 有时给你 view，有时给你 copy（和列类型、是不是连续块有关）。
    所以同一句代码：这次 side 写进了原 df，换一组数据又没写进去。
    pandas 发现你在「可能是 copy 的东西」上赋值，就警告 SettingWithCopyWarning。
    警告的意思不是语法错，是：这次赋值可能白写了。

    回测场景：你 print 了中间那张表，看见 side=1，以为标签赋好了；
    真正拿去下单的是原来的 df，那边 side 还是 0。

# loc 为什么能写 loc[行, 列]

loc 不是普通列，是一个索引器对象（df.loc）。
Python 规定：x[i] 就是 x.__getitem__(i)；x[i] = v 就是 x.__setitem__(i, v)。
方括号里的逗号会做成元组：loc[a, b] ≡ loc[(a, b)]。

所以 loc[mask, "side"] = 1 就是：
    df.loc.__setitem__((mask, "side"), 1)
pandas 认出「行选择器 + 列名」，走一条会写回原数据的路径。

df[mask] 走的是 DataFrame.__getitem__，先切片，再在切片上 ["side"]=，
第二次赋值已经不知道「这是原表的一部分」。

# 口条

过滤并赋值用 .loc[条件, 列] = 值。不要先切片再点列再赋值。
"""

import pandas as pd

df = pd.DataFrame({"ret": [0.01, -0.02, 0.03], "side": [0, 0, 0]})

# 行：ret>0 的布尔 Series；列："side"；一次写进原 df
df.loc[df["ret"] > 0, "side"] = 1
print("loc 之后\n", df)

# 对照：整列赋值（不按行过滤）
# df["side"] = 1

# 不要：两步切。中间表有时是 view、有时是 copy，赋值可能改不到原 df
# df[df["ret"] > 0]["side"] = 1
