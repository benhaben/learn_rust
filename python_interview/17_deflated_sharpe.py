"""进阶：试次越多，最大 Sharpe 期望越高；扫出来的 2 要打折。

运行：python 17_deflated_sharpe.py

# 为什么「Sharpe 2」不可信

一次随机策略，年化 Sharpe 的抽样分布中心在 0 附近。
你试了 N 组参数，只报最大的那个：最大值的期望 > 0，N 越大越吹。
这不是作弊代码，是多重检验。

AFML：Deflated Sharpe（DSR）= 把「零假设」从 0 改成「N 次里最大的期望 Sharpe」，
再看你的样本 Sharpe 还显著不显著。
本文件用模拟代替闭式：抽 N 条无 alpha 的日收益，各算 Sharpe，看最大值。

持有集 lock：最终只碰一次的样本，不能用来再调参（和 purge 不是同一件事）。

# 本文件

N=1 / 20 / 100 次随机策略，打印每次试验里最大的那个年化 Sharpe。
"""

import numpy as np

rng = np.random.default_rng(8)
T = 252


def max_sharpe(n_trials):
    # 每条策略：白噪声日收益，真 Sharpe=0。取 N 条里最大的年化 Sharpe。
    best = -np.inf
    for _ in range(n_trials):
        r = rng.normal(0, 0.01, T)
        sr = r.mean() / r.std(ddof=1) * np.sqrt(252)
        best = max(best, sr)
    return float(best)


print("真 alpha=0，一年噪声：")
for n in (1, 20, 100):
    print(f"  试 {n:3d} 次，最大 Sharpe ≈ {max_sharpe(n):.2f}")
print("口条：试次吹 Sharpe。持有集只碰一次。DSR 把零假设抬到「N 次最大值」。")
