"""绿皮后半：P 与 Q。真实漂移 μ 决定你看到的路径；欧式价格用 r。

运行：python 22_p_vs_q.py

P：dS = μ S dt + σ S dW     （真实世界，μ 含风险溢价）
Q：dS = r S dt + σ S dW     （风险中性，为了定价）
同一 σ。看涨期望贴现：用 μ 会算贵/算贱，不是无套利价格。
Girsanov：换测度 = 换漂移，波动矩阵不变。

口条：统计用 P，定价用 Q。μ 进预测，不进欧式 BS。
"""

import math
import numpy as np


def ncdf(x):
    return 0.5 * (1.0 + math.erf(x / math.sqrt(2.0)))


def bs_call(s, k, r, sig, t):
    d1 = (math.log(s / k) + (r + 0.5 * sig**2) * t) / (sig * math.sqrt(t))
    d2 = d1 - sig * math.sqrt(t)
    return s * ncdf(d1) - k * math.exp(-r * t) * ncdf(d2)


def mc_call(s, k, drift, r_disc, sig, t, rng, n=60_000):
    z = rng.normal(0, 1, n)
    st = s * np.exp((drift - 0.5 * sig**2) * t + sig * np.sqrt(t) * z)
    return float(np.exp(-r_disc * t) * np.maximum(st - k, 0).mean())


rng = np.random.default_rng(22)
s, k, r, mu, sig, t = 100.0, 100.0, 0.03, 0.12, 0.2, 1.0
closed = bs_call(s, k, r, sig, t)
under_q = mc_call(s, k, r, r, sig, t, rng)
under_p = mc_call(s, k, mu, r, sig, t, rng)
print(f"BS {closed:.3f}  Q 下 MC {under_q:.3f}  （应接近）")
print(f"用 μ 模拟再贴现 {under_p:.3f}  （不是无套利价，通常更大）")
print("口条：路径生成的漂是谁，决定你算的是价格还是「真实期望」。")
