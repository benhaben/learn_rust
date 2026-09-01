"""绿皮主题：几何布朗 ⇒ S_T 对数正态。均值 ≠ 中位数。

运行：python 15_lognormal.py

    ln S_T ~ N(ln S0 + (μ-½σ²)T, σ²T)
E[S_T] = S0 e^{μT}          被右尾拉高
中位数 = S0 e^{(μ-½σ²)T}   更小
「股价期望涨但多数路径涨不到均值」——不是矛盾，是偏度。

口条：对数正态右偏。谈期望说 e^{μT}；谈「典型路径」说中位数。
"""

import numpy as np

rng = np.random.default_rng(15)
s0, mu, sig, t = 100.0, 0.10, 0.3, 1.0
z = rng.normal(0, 1, 50_000)
st = s0 * np.exp((mu - 0.5 * sig**2) * t + sig * np.sqrt(t) * z)
mean_th = s0 * np.exp(mu * t)
med_th = s0 * np.exp((mu - 0.5 * sig**2) * t)
print(f"模拟均值 {st.mean():.2f}  公式 E={mean_th:.2f}")
print(f"模拟中位 {np.median(st):.2f}  公式 {med_th:.2f}")
print(f"P(S_T < 均值)={np.mean(st < mean_th):.3f}  （>1/2，右偏）")
print("口条：均值被大涨路径拉高；多数实现靠近中位数。")
