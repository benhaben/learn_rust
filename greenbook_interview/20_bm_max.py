"""绿皮后半：布朗运动的最大值。反射原理：P(M_t ≥ a) = 2 P(W_t ≥ a)（a>0）。

运行：python 20_bm_max.py

M_t = max_{0≤s≤t} W_s。路径连续、从 0 出发。
碰到 a 之后把后面反射，终点落在 a 以上和「终点在 a 以上」配成一对。
推论：P(曾经到过 a) 比 P(终点在 a 以上) 大约大一倍（对正 a）。

口条：问「有没有碰到」用反射，不是只用终点分布。障碍期权同一张图。
"""

import numpy as np

rng = np.random.default_rng(20)
t, n_step, n_path = 1.0, 800, 6000
dt = t / n_step
dW = rng.normal(0, np.sqrt(dt), size=(n_path, n_step))
W = np.cumsum(dW, axis=1)
m = W.max(axis=1)
a = 1.0
p_max = np.mean(m >= a)
p_end = np.mean(W[:, -1] >= a)
print(f"P(M_1≥{a})={p_max:.3f}  2P(W_1≥{a})={2 * p_end:.3f}  （应接近）")
print("口条：反射 ⇒ P(碰到 a)≈2P(终点≥a)。连续路径才会「路过」。")
