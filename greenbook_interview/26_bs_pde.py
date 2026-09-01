"""绿皮后半：BS 方程。Θ + rS Δ + ½σ²S² Γ = r C（无股息）。

运行：python 26_bs_pde.py

对冲组合瞬时无风险，只能赚 r。Γ 凸性靠 Θ 时间衰减来买单。
面试：短看涨 = 短 Γ、长 Θ（通常）；大动疼、不动收时间价值。

口条：先写 PDE。数值用中心差分验 Δ、Γ、Θ，残差应接近 0。
"""

import math
import numpy as np


def ncdf(x):
    return 0.5 * (1.0 + math.erf(x / math.sqrt(2.0)))


def bs_call(s, k, r, sig, t):
    if t <= 1e-12:
        return max(s - k, 0.0)
    d1 = (math.log(s / k) + (r + 0.5 * sig**2) * t) / (sig * math.sqrt(t))
    d2 = d1 - sig * math.sqrt(t)
    return s * ncdf(d1) - k * math.exp(-r * t) * ncdf(d2)


s, k, r, sig, t = 100.0, 100.0, 0.03, 0.2, 0.5
hs, ht = 0.05, 1e-4
c = bs_call(s, k, r, sig, t)
delta = (bs_call(s + hs, k, r, sig, t) - bs_call(s - hs, k, r, sig, t)) / (2 * hs)
gamma = (
    bs_call(s + hs, k, r, sig, t) - 2 * c + bs_call(s - hs, k, r, sig, t)
) / (hs**2)
# 公式里的 t 是日历：到期日固定，t 增大则剩余期限变短。
# Θ=∂C/∂日历 ≈ [C(τ-ht)-C(τ)]/ht，看涨通常为负。
theta = (bs_call(s, k, r, sig, t - ht) - c) / ht
lhs = theta + r * s * delta + 0.5 * sig**2 * s**2 * gamma
rhs = r * c
print(f"C={c:.4f}  Δ={delta:.4f} Γ={gamma:.4f} Θ={theta:.4f}")
print(f"PDE 左 {lhs:.5f}  右 rC={rhs:.5f}  差 {lhs - rhs:.5f}")
print("口条：Θ + rSΔ + ½σ²S²Γ = rC。Γ 凸性由负 Θ（时间衰减）买单。")
