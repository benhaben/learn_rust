"""必考：简单收益用于组合加权；对数收益多期可加。

运行：python 04_returns.py

# 两种收益

简单（百分比）  R = P_今 / P_昨 - 1 = (P_今 - P_昨) / P_昨
对数            r = log(P_今 / P_昨) = log(P_今) - log(P_昨)

恒等式：r = log(1+R)。小收益时 R≈r（log(1+x)≈x）。
本文件最后一行就是在验这条。

# log / exp 都是 e 为底

量化里说的 log 就是自然对数 ln，底是 e≈2.718，不是 10，也不是 2。
NumPy：np.log = ln；10 为底才是 np.log10；2 为底是 np.log2。
exp 是它的反函数：exp(x) = e^x，所以 exp(log(x)) = x（x>0）。

加出来的对数收益是「log 倍数」，不是财富倍数：
    r1+r2+…+rn = log(Pn / P0)           例如 log(1.21)≈0.1906
    exp(r1+…+rn) = Pn / P0              例如 exp(0.1906)≈1.21
两个数本来就不该一样：一个在 log 空间，一个在价格空间。
要回到「钱变成几倍」，对加总后的 r 做一次 exp。

# 为什么对数多期可加（望远镜）

两天：
    r1 + r2
  = (log P1 - log P0) + (log P2 - log P1)
  = log P2 - log P0
  = log(P2 / P0)

中间的 log P1 消掉。任意多期：
    r1+r2+…+rn = log(Pn / P0)

简单收益不能这么加：
    R1+R2  ≠  P2/P0 - 1
要还原价格，必须连乘：
    (1+R1)(1+R2)…(1+Rn) = Pn / P0
所以回测净值用 (1+R).cumprod()，不是 R.cumsum()。

# 为什么组合必须用简单收益

钱是可加的。两只股票各一半：
    组合盈亏 = 0.5 * 票A盈亏 + 0.5 * 票B盈亏
    R_组合 = w_A * R_A + w_B * R_B

对数对权重没有这条：
    log(w_A P_A + w_B P_B) ≠ w_A log P_A + w_B log P_B
把日对数收益加权当组合收益，面试会扣。

用法分工：
    回测净值、组合、换手成本  → 简单 R
    多期累加、统计建模、近似正态 → 对数 r

# 本文件在做什么

价格 100 → 101 → 99 → 102。
pct_change() 算各期 R（第一格没有「昨」，是 NaN）。
log(px).diff() 算各期 r。
(1+R).cumprod() 从 1 块本金滚净值；第一期 R 填 0，净值从 1 起。
"""

import numpy as np
import pandas as pd

# 四根收盘价。第一根没有「上一根」，后面算收益时那一格会是 NaN。
px = pd.Series([100.0, 101.0, 99.0, 102.0])

# 简单：每一格 / 前一格 - 1。100→101 是 +1%，101→99 是约 -1.98%。
R = px.pct_change()

# 对数：先对价格取 log，再相邻相减。就是 log(今)-log(昨)。
# 多期把这些 r 加起来 = log(末价)-log(首价)，中间消掉。
r = np.log(px).diff()

# 净值：本金 1，每期乘 (1+R)。第一期 R 是 NaN，当成「还没交易」填 0。
# cumprod = 累积连乘，对应 (1+R1)(1+R2)…，还原价格路径（相对第一根）。
nav = (1 + R.fillna(0)).cumprod()

print("简单收益\n", R)
print("对数收益\n", r)
print("净值\n", nav)

# r = log(1+R)。两边 dropna 去掉第一根 NaN。
print("R 与 r 接近", np.allclose(r.dropna(), np.log(1 + R.dropna()), atol=1e-12))

# 对数：加总 = 首尾比的 log。简单：加总 ≠ 首尾简单收益；连乘才还原。
print("对数相加", float(r.sum()), "应等于", float(np.log(px.iloc[-1] / px.iloc[0])))
print("简单相加", float(R.sum()), "≠ 总简单", float(px.iloc[-1] / px.iloc[0] - 1))
print("简单连乘", float((1 + R.dropna()).prod()), "应等于", float(px.iloc[-1] / px.iloc[0]))
# exp 是 ln 的反函数：exp(Σr) 才是财富倍数，和上面的连乘同一回事
print("exp(对数相加)", float(np.exp(r.sum())), "应等于连乘")
