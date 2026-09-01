"""绿皮后半：次序统计量。n 个 U(0,1) 的最小期望是 1/(n+1)。

运行：python 18_order_stat.py

    X_(1) < … < X_(n)  是排好序的样本
均匀：[0,1] 上 n 个点把区间切成 n+1 段，平均一样长。
所以 E[min]=E[max 的对称]=1/(n+1)，E[max]=n/(n+1)。

口条：先说分布，再排序。均匀最好算；一般分布用 CDF 变换。
"""

import numpy as np

rng = np.random.default_rng(18)
n, trials = 5, 30_000
u = rng.random((trials, n))
mn, mx = u.min(axis=1), u.max(axis=1)
print(f"n={n}  E[min] {mn.mean():.3f}  公式 {1 / (n + 1):.3f}")
print(f"       E[max] {mx.mean():.3f}  公式 {n / (n + 1):.3f}")
print("口条：n 个 U(0,1)，E[最小]=1/(n+1)。像 n+1 段绳子一样长。")
