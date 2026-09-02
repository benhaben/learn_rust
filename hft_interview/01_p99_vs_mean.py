"""必考：延迟看 p99，不看均值。和看 MaxDD 不看平均收益同一套思维。

运行：python 01_p99_vs_mean.py

# 人话
大部分 tick 很快（8μs），少数几次分配/抢锁/缺页变成 80μs。
均值被「大多数」拉好看；被抢先、被捡走的是尾巴。

# 目的
面试能解释：为什么报平均延迟是在撒谎。

# 场景
热路径性能、SLA、和「策略平均每天赚、一次回撤爆仓」对照。
"""

import numpy as np

rng = np.random.default_rng(1)
# 95% 快路径 ~8μs，5% 慢路径 ~80μs。
n = 20_000
fast = rng.normal(8.0, 1.0, int(n * 0.95))
slow = rng.normal(80.0, 8.0, n - len(fast))
lat = np.clip(np.concatenate([fast, slow]), 0.1, None)

print(f"均值 {lat.mean():.1f} μs   中位 {np.median(lat):.1f} μs")
print(f"p95 {np.percentile(lat, 95):.1f}  p99 {np.percentile(lat, 99):.1f}  max {lat.max():.1f}")
print("口条：均值好看，p99 才是被抢先的时刻。")
