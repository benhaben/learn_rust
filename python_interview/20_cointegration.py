"""进阶：配对看协整，不是只看价格相关。

运行：python 20_cointegration.py

# 人话
相关 = 两只价格一起涨（牛市里很容易高，价差仍可能越走越开）。
协整 = 各自都是乱走（不平稳），但某个线性组合（价差）会回到均值（平稳）。
只有价差平稳，才谈得上「贵了空、便宜了多」的配对。

# 目的
避免把「两只都在涨」当成能配对交易。相关高 ≠ 能套利。

# 场景
配对交易、统计套利选对。正式检验用 ADF / Johansen（本文件不引入 statsmodels，看数字说话）。
这和 16 的分数差分不同：这里处理的是两只价格的价差，不是单只价格进模型。

# 相关 vs 协整

两只都在涨，价格相关可以很高，价差仍可能越走越开（都是随机游走）。
协整：各自非平稳，但某个线性组合（价差）平稳，才谈得上均值回复配对。

本文件：B = A + 平稳价差。价格相关虚高；价差在 0 附近晃。
没有 statsmodels，不做正式 ADF / Johansen，看数字说话。

# 本文件

造 A 随机游走，B=A+噪声价差。打印价格相关、价差相关（应接近 0）、价差均值。
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(11)
# A：随机游走（对数价）。B：同一条游走 + 平稳价差。
a = np.cumsum(rng.normal(0, 0.01, 120))
spread = np.zeros(120)
for i in range(1, 120):
    spread[i] = 0.7 * spread[i - 1] + rng.normal(0, 0.005)
b = a + spread
A, B = pd.Series(np.exp(a)), pd.Series(np.exp(b))

print("价格相关（两只一起爬，容易虚高）", float(A.corr(B)))
print("价差一阶相关（应远小于价格相关）", float(pd.Series(spread).diff().corr(pd.Series(a).diff())))
print("价差均值", float(spread.mean()), "末值", float(spread[-1]))
print("口条：相关高不等于能配对；要价差平稳（协整）。")
