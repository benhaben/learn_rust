"""绿皮后半：泊松过程。独立增量、一次跳一个，等待时间是指数。

运行：python 17_poisson.py

N_t ~ Poisson(λt)：E=Var=λt。
相邻到达间隔 ~ Exp(λ)，无记忆（接 06）。
面试：到第 n 次的时间是 Gamma(n,λ)；两段不交叠的计数独立。

口条：数事件用泊松，等下一次用指数。同一条过程的两种看法。
"""

import numpy as np

rng = np.random.default_rng(17)
lam, t, n_path = 3.0, 2.0, 6_000
# 方法 A：直接抽 N_t。
nt = rng.poisson(lam * t, n_path)
# 方法 B：一直加指数间隔，看 t 之前来了几次。
counts = []
for _ in range(n_path):
    acc, k = 0.0, 0
    while True:
        acc += rng.exponential(1.0 / lam)
        if acc > t:
            break
        k += 1
    counts.append(k)
counts = np.array(counts)

print(f"Poisson  E={nt.mean():.3f} Var={nt.var():.3f}  （应≈{lam * t}）")
print(f"间隔累加 E={counts.mean():.3f} Var={counts.var():.3f}")
print("口条：N_t~Pois(λt)；等待 ~Exp(λ)。数和等是一件钟的两种读法。")
