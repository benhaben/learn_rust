"""绿皮主题：Delta。价格对 S 的一阶，无股息欧式看涨 Δ=Φ(d1)。

运行：python 14_delta_bump.py

有限差分：Δ ≈ (C(S+h)-C(S-h)) / (2h)
和闭式 Φ(d1) 对上。面试还要会：对冲卖 1 份 call 要买 Δ 份标的；
Γ 是 Δ 再对 S 求导，短伽马在大动时疼。

口条：Delta 是线性估计。Bump 要对称，h 太小有噪声、太大有高阶误差。
"""

import math
import numpy as np


def ncdf(x):
    return 0.5 * (1.0 + math.erf(x / math.sqrt(2.0)))


def bs_call(s, k, r, sig, t):
    d1 = (math.log(s / k) + (r + 0.5 * sig**2) * t) / (sig * math.sqrt(t))
    d2 = d1 - sig * math.sqrt(t)
    return s * ncdf(d1) - k * math.exp(-r * t) * ncdf(d2), ncdf(d1)


s, k, r, sig, t = 100.0, 100.0, 0.03, 0.2, 1.0
c, delta = bs_call(s, k, r, sig, t)
h = 0.01
c_up, _ = bs_call(s + h, k, r, sig, t)
c_dn, _ = bs_call(s - h, k, r, sig, t)
bump = (c_up - c_dn) / (2 * h)
print(f"C={c:.4f}  Φ(d1)={delta:.4f}  中心差分 {bump:.4f}")
print("口条：欧式看涨 Δ=Φ(d1)∈(0,1)。对冲：短看涨、长 Δ 份标的。")
