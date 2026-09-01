"""绿皮主题：Black–Scholes。风险中性下，贴现期望 = 闭式。

运行：python 13_bs_mc.py

风险中性：dS = r S dt + σ S dW（测度为 Q，漂是 r 不是 μ）。
路径：S_T = S0 exp((r-½σ²)T + σ√T Z)，Z~N(0,1)
看涨：e^{-rT} E[max(S_T-K,0)]

口条：先换测度（漂改 r），再期望，再贴现。μ 不进欧式价格。
"""

import math
import numpy as np


def ncdf(x):
    return 0.5 * (1.0 + math.erf(x / math.sqrt(2.0)))


def bs_call(s, k, r, sig, t):
    d1 = (math.log(s / k) + (r + 0.5 * sig**2) * t) / (sig * math.sqrt(t))
    d2 = d1 - sig * math.sqrt(t)
    return s * ncdf(d1) - k * math.exp(-r * t) * ncdf(d2)


rng = np.random.default_rng(13)
s, k, r, sig, t = 100.0, 100.0, 0.03, 0.2, 1.0
z = rng.normal(0, 1, 80_000)
st = s * np.exp((r - 0.5 * sig**2) * t + sig * np.sqrt(t) * z)
mc = np.exp(-r * t) * np.maximum(st - k, 0).mean()
closed = bs_call(s, k, r, sig, t)
print(f"闭式 {closed:.4f}  MC {mc:.4f}")
print("口条：价格是 Q 下贴现期望。真实 μ 只影响 P 测度路径，不进欧式 BS。")
