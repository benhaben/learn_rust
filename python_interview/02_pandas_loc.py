"""必考：.loc 一次写进原表；链式 df[mask][col]= 可能改在副本上

运行：python 02_pandas_loc.py

# 人话
`.loc[哪些行, 哪一列] = 值`：一次说清楚改哪里，写进原表。
`df[条件]["列"] = 值`：先切再写。中间那张表有时是原表窗口，有时是副本，
写上去可能改不到你真正拿去下单的 df。

# 目的
标签、仓位、方向必须写进「后面计算用的那张表」，不能只改了一份复印件。

# 场景
按收益正负打多空、按信号赋仓、任何「过滤一部分行再改某一列」。
按名字取行用 loc，按第几行用 iloc。

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

# loc 和 iloc 差在哪

都是 [行, 列] 一次定位。差别只在「行/列」按什么认。

    loc  = 标签（index / columns 上贴的名字）
    iloc = 位置（第几行第几列，从 0 数，和 C 数组下标一样）

默认没改过 index 时，行标签就是 0,1,2，碰巧等于位置，
loc[0] 和 iloc[0] 看起来一样。删一行或换成日期之后就不一样了。

    行还在：标签 0, 2（中间的 1 没了）
    loc[2]     名字叫 2 的那一行（现在是表里第 2 条）
    iloc[2]    第 3 行 → 越界
    iloc[1]    现在的第 2 行，标签仍是 2

切片开闭也不一样：
    loc[0:2]   标签 0 到 2，两端都包含（按名字取范围）
    iloc[0:2]  位置 0、1，不含 2（和 Python / NumPy 半开区间一样）

列同理：loc[:, "side"] 按列名；iloc[:, 1] 按第 1 列。
布尔条件（ret>0）是标签语义，用 loc，不要用 iloc。

# 口条

过滤并赋值用 .loc[条件, 列] = 值。不要先切片再点列再赋值。
按名字用 loc，按第几行用 iloc。默认 0,1,2 只是碰巧两种写法重合。
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

# loc=标签，iloc=位置。删掉 index=1 之后，剩下标签 0、2
df2 = pd.DataFrame({"ret": [0.01, -0.02, 0.03], "side": [0, 0, 0]})
df2 = df2.drop(1)
print("删行后 index", list(df2.index))
print("loc[2] 标签 2\n", df2.loc[2])
print("iloc[1] 现在的第 2 行\n", df2.iloc[1])
# df2.iloc[2]  # IndexError：只剩 2 行，没有第 3 行
