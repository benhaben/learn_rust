"""进阶：分数差分 — 要平稳，又比一阶差分少丢价格记忆。

运行：python 16_frac_diff.py

# 一阶差分的问题

r = logP_t - logP_{t-1} 很平稳，但水平没了：100 和 200 看起来一样。
有些因子（估值距离、通道位置）还需要一点「记得以前的价」。

# 分数差分 d ∈ (0,1)

权重递推（AFML）：
    w0 = 1
    w_k = -w_{k-1} * (d-k+1) / k

d=1 时只剩 [1, -1]，就是普通差分。
d=0.4 时后面的权重慢慢衰减，旧价格还留一点。
选 d：小到序列通过平稳检验，又尽量靠近 0（多留记忆）。
本文件只打印权重，不跑 ADF（不引入 statsmodels）。

# 本文件

打印 d=1 和 d=0.4 的前几项权重；对假价格做卷积，看分数差分还留不留水平。
"""

import numpy as np
import pandas as pd

def frac_weights(d, size):
    # w[k] 乘的是 P_{t-k}。d=1 → [1, -1, 0, 0, …]
    w = [1.0]
    for k in range(1, size):
        w.append(-w[-1] * (d - k + 1) / k)
    return np.array(w)


w1 = frac_weights(1.0, 8)
w04 = frac_weights(0.4, 8)
print("d=1 权重（普通差分）", np.round(w1, 3))
print("d=0.4 权重（慢慢衰减）", np.round(w04, 3))

rng = np.random.default_rng(7)
# 带漂移的价格，水平会爬。
logp = np.cumsum(rng.normal(0.001, 0.01, 80))
px = 100 * np.exp(logp)
w = frac_weights(0.4, 20)
# 有效样本从第 20 格起：和过去 20 根加权。
fd = np.convolve(px, w, mode="valid")
print("价格首尾", float(px[19]), float(px[-1]))
print("分数差分首尾", float(fd[0]), float(fd[-1]), "（比价格短，水平被压住但仍相关）")
print("一阶差分均值", float(np.diff(px).mean()), "已经几乎不带水平")
print("口条：d=1 全丢记忆；d∈(0,1) 折中平稳和记忆。")
