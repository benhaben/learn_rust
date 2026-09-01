"""绿皮主题：标准布朗运动 W_t。连续时间的随机游走极限。

运行：python 10_brownian.py

    W_0=0，增量独立，W_t-W_s ~ N(0, t-s)，路径连续（几乎处处）。
离散：每步 ±sqrt(dt)，或 N(0,dt)。面试要会：E[W_t]=0，Var(W_t)=t，
不是 t²。hitting、反射、极值是后续题。

口条：方差跟时间走，标准差跟根号时间走。和日波动×√252 同一件事。
"""

import numpy as np

rng = np.random.default_rng(10)
t, n_step, n_path = 1.0, 500, 4000
dt = t / n_step
# 每步 N(0, dt)，累加 = W_t。
dW = rng.normal(0, np.sqrt(dt), size=(n_path, n_step))
W = dW.sum(axis=1)

print(f"E[W_1] {W.mean():.3f}（应≈0）  Var {W.var():.3f}（应≈{t}）")
print(f"sd {W.std():.3f}（应≈{np.sqrt(t):.3f}）")
print("口条：W_t ~ N(0,t)。方差加时间，波动加根号时间。")
