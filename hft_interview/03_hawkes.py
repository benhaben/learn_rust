"""必考：Hawkes = 底噪 + 过去每笔余波。泊松不会成簇。

运行：python 03_hawkes.py

# 人话
泊松：强度常数，到达均匀。
Hawkes：每来一笔，λ 跳一下再按 e^{-βt} 衰减。一笔大热闹会「传染」。

# 目的
能画 λ(t)，能说分支比 n=α/β < 1。

# 场景
成交量扎堆、假订单流压测试。方向另说。聚到 2h 上用会死。
"""

import numpy as np

rng = np.random.default_rng(3)
mu, alpha, beta = 0.4, 0.6, 1.2  # n=α/β=0.5 < 1
t_end = 40.0

# 最简单的 Ogata 细化：在上界里投点，再按 λ(t)/bound 接受。
times = []
t, lam_max = 0.0, mu + 8.0
while t < t_end:
    t += rng.exponential(1.0 / lam_max)
    if t >= t_end:
        break
    # λ(t) = μ + Σ α e^{-β(t-ti)}
    decay = np.exp(-beta * (t - np.array(times))) if times else 0.0
    lam = mu + (alpha * np.sum(decay) if times else 0.0)
    if rng.random() < lam / lam_max:
        times.append(t)

times = np.array(times)
gaps = np.diff(times)
# 簇：间隔特别短的比例，对比同均值泊松。
poi = rng.exponential(t_end / max(len(times), 1), size=len(gaps))
print(f"Hawkes 点数 {len(times)}  分支比 α/β={alpha / beta:.2f}")
print(f"间隔<0.3 的比例  Hawkes {np.mean(gaps < 0.3):.2f}  同均值泊松 {np.mean(poi < 0.3):.2f}")
print("口条：余波让短间隔扎堆。泊松一样的均值，没有这种簇。")
