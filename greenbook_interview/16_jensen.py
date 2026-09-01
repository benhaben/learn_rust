"""绿皮后半：Jensen。凸函数外面取期望 ≥ 先取期望再套函数。

运行：python 16_jensen.py

    f 凸  ⇒  E[f(X)] ≥ f(E[X])
e^x、x²、max(S-K,0) 都凸。所以：
  E[e^X] > e^{E[X]}（除非 X 退化）
  看涨价格 > 用远期当确定值算的内在（波动有价值）

口条：凸的惩罚两边、奖励中间反过来——波动让 E[payoff] 变大。凹则相反。
"""

import numpy as np

rng = np.random.default_rng(16)
x = rng.normal(0, 1, 40_000)
ex_f = np.exp(x).mean()
f_ex = np.exp(x.mean())
print(f"E[e^X]={ex_f:.3f}  e^{{E[X]}}={f_ex:.3f}  差 {ex_f - f_ex:.3f} > 0")

# 看涨：S 对数正态。E[max(S-K,0)] vs max(E[S]-K,0)
s = 100 * np.exp(0.1 + 0.3 * rng.normal(0, 1, 40_000))
k = 100.0
pay = np.maximum(s - k, 0).mean()
intr = max(s.mean() - k, 0)
print(f"E[(S-K)+]={pay:.2f}  (E[S]-K)+={intr:.2f}  期权时间价值在这个差里")
print("口条：凸 ⇒ 期望和函数不能换序。波动本身值钱。")
