"""绿皮主题：Itô。(dW)² 在和的意义上等于 dt，不是可以扔掉的二阶小量。

运行：python 11_ito_dw2.py

普通微积分 (dt)²=0。布朗运动一步是 sqrt(dt) 量级，
平方之后是 dt，N 步加起来是 T，不会消失。
所以 d(W²)=2W dW + dt，多出那一项 dt（Itô 修正）。

口条：对 W 做泰勒必须留 (dW)²→dt。BS 的 ½σ²S² 项从这里来。
"""

import numpy as np

rng = np.random.default_rng(11)
t, n_step, n_path = 1.0, 1000, 2000
dt = t / n_step
dW = rng.normal(0, np.sqrt(dt), size=(n_path, n_step))
# 每条路径 Σ (ΔW)² 应接近 T=1。
quad = (dW**2).sum(axis=1)

print(f"Σ(ΔW)² 均值 {quad.mean():.3f}（应≈{t}）  标准差 {quad.std():.3f}")
print(f"Σ(Δt) 当然是 {n_step * dt:.3f}")
print("口条：(dW)²~dt。二次变差是 t，不是 0。")
