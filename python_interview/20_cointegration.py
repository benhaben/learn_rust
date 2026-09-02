"""进阶：配对看协整，不是只看价格相关。

运行：python 20_cointegration.py

# 人话
相关 = 两只价格一起涨（牛市里很容易高，价差仍可能越走越开）。
协整 = 各自都是乱走（不平稳），但某个线性组合（价差）会回到均值（平稳）。
只有价差平稳，才谈得上「贵了空、便宜了多」的配对。

# 目的
避免把「两只都在涨」当成能配对交易。相关高 ≠ 能套利。

# 场景
配对交易、统计套利选对。正式检验用 ADF / Johansen。
这和 16 的分数差分不同：这里处理的是两只价格的价差，不是单只价格进模型。

# 相关 vs 协整

两只都在涨，价格相关可以很高，价差仍可能越走越开（都是随机游走）。
协整：各自非平稳，但某个线性组合（价差）平稳，才谈得上均值回复配对。

# ADF 是什么（Augmented Dickey-Fuller）

问的就一件事：这条序列是随机游走（有单位根、不平稳），还是会回到均值（平稳）。
随机游走：今天 = 昨天 + 噪声，走偏了没人拉，位置越走越远。
平稳：已经偏高了，下一步更容易往回走。

检验写成回归：

    Δy_t = μ + γ y_{t−1} + ε

Δy_t = 今天减昨天（这一步走了多远）。y_{t−1} = 昨天在哪。
γ ≈ 0：昨天位置不影响下一步，还是乱走。
γ < 0 且够负：偏高了下一步往下，会被拉回。

adf_t 就是 γ 的 t（估计 / 标准误）。越负越像平稳。
不要拿普通 t 的 −1.96 来比：原假设是「有单位根」，分布不是正态，临界更严。
有常数、无滞后：5% 大约 −2.89。价格 −0.86 过不了；残差 −7.24 远小于 −3.3。

Augmented（增广）：残差自己还有相关时，t 会不准。
完整 ADF 再往右边加 Δy_{t−1}、Δy_{t−2}… 把相关吸掉。
本文件为了能手算，没加滞后，是最简 DF，不是完整 ADF。

ADF 一次只看一条序列。Engle-Granger 是先把两只收成一条残差，再对残差做 ADF。

# 怎么看出协整（Engle-Granger，本文件实际做的）

两步，缺一步都不叫协整：

    1) 各自是随机游走：价格自己的 ADF 过不了（t 不够负）。
    2) 用一只回归另一只：B ≈ α + β A，残差 e = B − α − βA。
       若 e 的 ADF 够负 → 价差平稳 → 协整。β 就是对冲比。

只看「B−A 晃不晃」不够：对冲比不一定是 1。先 OLS 再验残差。
面试记住：价格过不了、残差过了，才叫协整。残差门槛更严（约 −3.3），因为 β 是估出来的。

对照 C = A + 另一条随机游走：共享同一条趋势，价格相关照样虚高，
但多出来的游走让价差本身也不平稳，残差 ADF 过不了。

# 本文件

造 A 随机游走。B = A + 平稳 AR(1) 价差（协整）。C = A + 另一条游走（只同涨）。
各自 OLS 对冲，打印价格相关、价格 ADF、残差 ADF、残差前后半段均值。
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(10)
n = 250
# A：随机游走（对数价）。
a = np.cumsum(rng.normal(0.0008, 0.01, n))
# 协整对：B = A + 平稳价差。φ=0.7 < 1，偏离会衰减。
spread = np.zeros(n)
for i in range(1, n):
    spread[i] = 0.7 * spread[i - 1] + rng.normal(0, 0.005)
b = a + spread
# 只同涨：C 和 A 共享同一条游走，再叠一条独立游走。相关虚高，价差不平稳。
c = a + np.cumsum(rng.normal(0, 0.012, n))
A = pd.Series(np.exp(a), name="A")
B = pd.Series(np.exp(b), name="B")
C = pd.Series(np.exp(c), name="C")


def adf_t(y):
    # 最简 ADF：Δy = μ + γ y_{t-1} + ε。返回 γ 的 t。
    # 越负越像平稳。约 −2.89 过 5%（单序列、有常数、无滞后）。
    y = np.asarray(y, dtype=float)
    dy = np.diff(y)
    ylag = y[:-1]
    X = np.column_stack([np.ones(len(dy)), ylag])
    coef, *_ = np.linalg.lstsq(X, dy, rcond=None)
    resid = dy - X @ coef
    # se(γ)：残差方差 / Σ(y_{t-1}−ȳ)²。自由度 n−2（μ 和 γ）。
    se = np.sqrt((resid @ resid) / (len(dy) - 2) / np.sum((ylag - ylag.mean()) ** 2))
    return float(coef[1] / se)


def engle_granger(x, y):
    # 第一步：y ≈ α + β x。β 是对冲比（1 份 y 对冲 β 份 x）。
    x = np.asarray(x, dtype=float)
    y = np.asarray(y, dtype=float)
    X = np.column_stack([np.ones(len(x)), x])
    alpha, beta = np.linalg.lstsq(X, y, rcond=None)[0]
    resid = y - (alpha + beta * x)
    return float(alpha), float(beta), resid


def report(name, x, y):
    alpha, beta, resid = engle_granger(x, y)
    mid = len(resid) // 2
    print(f"\n{name}")
    print(f"  价格相关 {float(pd.Series(x).corr(pd.Series(y))):.3f}   对冲比 β {beta:.3f}")
    print(f"  价格 ADF  x {adf_t(x):+.2f}  y {adf_t(y):+.2f}   （应过不了，各自乱走）")
    print(f"  残差 ADF      {adf_t(resid):+.2f}   （协整要够负，大约 < −3.3）")
    print(f"  残差前半/后半均值 {resid[:mid].mean():+.4f} / {resid[mid:].mean():+.4f}")


print("ADF 5% 大约 −2.89（单序列）；残差门槛更严，约 −3.3")
report("协整对 A,B（B=A+平稳价差）", A, B)
report("只同涨 A,C（C=A+另一条游走）", A, C)
print("\n口条：相关高不等于能配对；要 OLS 残差平稳（协整）。")
