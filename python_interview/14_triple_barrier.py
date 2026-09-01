"""进阶：三重屏障标签 = 先碰到止盈 / 止损 / 超时。

运行：python 14_triple_barrier.py

# 三个边

上屏障：价格涨到 入场 × (1+pt) → 标签 +1（止盈）
下屏障：跌到 入场 × (1-sl) → 标签 -1（止损）
垂直屏障：走满 max_h 根还没碰到 → 标签 0 或按到期收益符号

pt、sl 要随当时波动缩放（本文件 1.5×近 20 日 σ），否则牛市全是上障。

# 和「下一根收益」差在哪

09 的 y=r.shift(-1) 是回归明天涨多少。
三重屏障是分类：这段持有里先碰到谁。利润在尾部，准确率 55% 仍可能亏
（对了很多小波动，错了几次大止损）。评估看扣费后的钱，不看 accuracy。

# 本文件

假价格 → 每天一个事件 → 往前扫，记下先碰到哪边。
每天都当事件，标签窗口重叠——这就是 08 要 purge 的原因。实盘事件应稀疏。
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(5)
# 对数收益滚价格，起点 100。和 09 同一套假法。
px = pd.Series(100 * np.exp(np.cumsum(rng.normal(0, 0.01, 80))))
vol = np.log(px).diff().rolling(20).std()

# 屏障宽度 = 1.5 倍近期日波动（简单收益近似：σ 小时 ≈ 对数）。
pt = sl = 1.5 * vol
max_h = 10
labels = []

# 从有波动的那天起，每个 t 当一次入场（演示；实盘事件稀疏得多）。
for t in range(20, len(px) - max_h):
    p0 = px.iloc[t]
    up, dn = p0 * (1 + pt.iloc[t]), p0 * (1 - sl.iloc[t])
    path = px.iloc[t + 1 : t + 1 + max_h]
    lab, hit = 0, max_h
    for i, p in enumerate(path, start=1):
        if p >= up:
            lab, hit = 1, i
            break
        if p <= dn:
            lab, hit = -1, i
            break
    labels.append((t, lab, hit))

s = pd.DataFrame(labels, columns=["t", "label", "bars_to_hit"])
print("标签计数\n", s["label"].value_counts().sort_index())
print("前 8 个事件（1=先止盈 -1=先止损 0=超时）\n", s.head(8))
print("口条：三个边=止盈/止损/超时；屏障按波动缩放。")
