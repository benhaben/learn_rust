"""绿皮后半（算法）：两个均匀数变成两个独立正态（Box–Muller）。

运行：python 28_box_muller.py

    Z1 = √(-2 ln U1) cos(2π U2)
    Z2 = √(-2 ln U1) sin(2π U2)
面试还常问：只要公平硬币怎么得到 1..n；拒绝采样。这里只验均值 0、方差 1、不相关。

口条：均匀 → 半径用指数（-2ln U）、角度均匀。一次出一对独立 N(0,1)。
"""

import numpy as np

rng = np.random.default_rng(28)
u1, u2 = rng.random(40_000), rng.random(40_000)
r = np.sqrt(-2.0 * np.log(u1))
th = 2.0 * np.pi * u2
z1, z2 = r * np.cos(th), r * np.sin(th)
print(f"Z1 均值 {z1.mean():.3f}  方差 {z1.var():.3f}")
print(f"Z2 均值 {z2.mean():.3f}  方差 {z2.var():.3f}")
print(f"相关 {np.corrcoef(z1, z2)[0, 1]:.3f}（应≈0）")
print("口条：Box–Muller 用两个 U(0,1) 造一对独立正态。")
