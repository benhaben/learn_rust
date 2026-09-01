"""绿皮后半：计息。年名义 r，复利 m 次 → 连续是极限。

运行：python 23_compound.py

    (1 + r/m)^{m t}  →  e^{r t}     m→∞
远期价格（无股息）：F = S e^{rT}（连续）或 S (1+r)^T（年复）。
面试还要会：同一有效年利率，名义季度利率更低；贴现因子相乘。

口条：先问复利约定。连续是 e^{rt}，不是 1+rt（那是单利）。
"""

import numpy as np

r, t = 0.12, 1.0
print("名义 12%、一年：")
for m in (1, 2, 4, 12, 365):
    print(f"  {m:3d} 次  {(1 + r / m) ** (m * t):.6f}")
print(f"  连续    {np.exp(r * t):.6f}")

s, t2 = 100.0, 0.5
print(f"\nS={s}  半年远期（连续 r） F={s * np.exp(r * t2):.3f}")
print("口条：次数↑ 靠近 e^{rt}。远期无股息 F=S·贴现的倒数。")
