"""必考：冲击。你买多少，价格推多少。回测按 mid 成交 = 假设冲击为零。

运行：python 04_kyle.py

# 人话
线性 Kyle：Δmid ≈ λ × 有符号成交量。
临时冲击也常写成 sign(Q) * c * |Q|^0.5。
大单不能拆成「很多笔按当时 mid 成交」。

# 目的
面试默写一种冲击，并说出回测怎么假。

# 场景
拆单、VWAP、做市被扫。平方根比线性更贴大单，小单两者接近。
"""

import numpy as np

rng = np.random.default_rng(4)
q = rng.normal(0, 1, 8) * 10  # 有符号量
lam, c = 0.02, 0.05
kyle = lam * q
sqrt_imp = np.sign(q) * c * np.sqrt(np.abs(q))
print("Q      Kyle λQ   平方根冲击")
for a, b, d in zip(q, kyle, sqrt_imp):
    print(f"{a:7.2f}  {b:8.3f}  {d:8.3f}")
print("口条：λ 是线性渗价；大单用平方根。mid 成交 = λ=0。")
