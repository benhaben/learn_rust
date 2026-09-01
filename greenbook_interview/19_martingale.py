"""绿皮后半：鞅。公平游戏：下一步期望等于现在（已知历史）。

运行：python 19_martingale.py

    E[M_{t+1} | F_t] = M_t
无漂移的随机游走、风险中性下的贴现资产，都是鞅。
停时（规则只看已经发生的）：可选停定理 —— 好的停时 E[M_τ]=E[M_0]。
反例：有限赌本、或「一直等到赚钱」没有可积，期望会漂。

口条：鞅 = 条件期望不变。定价：贴现价格在 Q 下是鞅。
"""

import numpy as np

rng = np.random.default_rng(19)
# 公平 ±1 游走：E[X_{k+1}|X_k]=X_k。
steps = rng.choice([-1.0, 1.0], size=(8000, 40))
x = np.cumsum(steps, axis=1)
# 第 10 步之后再走 10 步，增量均值应≈0（与当前位置无关）。
inc = x[:, 19] - x[:, 9]
print(f"公平游走  后 10 步增量均值 {inc.mean():.4f}（应≈0）")

# 坏停时：破产在 0、封顶在 +5，从 2 出发 —— 有界所以可选停仍成立。
# 无界「赚到再停」会破可积，这里不演示。
start = 2
hit = []
for _ in range(4000):
    pos = start
    for _ in range(10_000):
        pos += 1 if rng.random() < 0.5 else -1
        if pos <= 0 or pos >= 5:
            hit.append(pos)
            break
print(f"停在 {{0,5}} 的均值 {np.mean(hit):.3f}（应≈起点 {start}）")
print("口条：E[下一刻|现在]=现在。停得合法，期望还是起点。")
