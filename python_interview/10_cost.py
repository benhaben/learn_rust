"""必考：仓位必须滞后；净收益扣换手成本。

运行：python 10_cost.py

# 为什么仓位要 shift(1)

今天的涨跌 r[t] 发生在今天。你能带着过夜的，是昨天收盘就定好的仓 pos[t-1]。
今天盘中才决定的 pos[t]，不能去乘今天的 r[t]——那是用未来仓位赚已经发生的收益。

    毛收益 = 昨仓 × 今收益
    gross = pos.shift(1) * r

# 成本扣在换手上

仓位从 0→1 或 1→0 才发生买卖。|Δ仓| 是换了多少手。
单边成本 = 佣金 + 滑点（本文件各 5bp，合计 10bp）。
买卖各付一次，所以年成本粗算：年换手 × 单边 × 2。

    cost = |pos[t]-pos[t-1]| × (fee+slip)
    净收益 = 毛 − 成本

# 本文件四天（pos 是「当日想要的目标仓」）

    日    r      pos   昨仓    毛         |Δ仓|   成本      净
    0   +1%     0    NaN    NaN        NaN    NaN      NaN   没有昨仓
    1   -1%     1     0     0          1      10bp    -10bp  今天才买上，吃不到今天涨跌
    2   +2%     1     1    +2%         0       0      +2%    昨已满仓，无换手
    3    0      0     1     0          1      10bp    -10bp  今天平仓，付成本
"""

import numpy as np
import pandas as pd

# 四天简单收益。演示用，不是真行情。
r = pd.Series([0.01, -0.01, 0.02, 0.00])

# 当日收盘想要的目标仓：空 → 满 → 满 → 空。不是「已经带着过夜的仓」。
pos = pd.Series([0.0, 1.0, 1.0, 0.0])

# 单边 5bp 佣金 + 5bp 滑点。大单冲击另说，这里合成 10bp。
fee, slip = 0.0005, 0.0005

# 昨仓 × 今收益。第 0 格 shift 出 NaN。禁止写 pos * r（用了当天才定的仓）。
gross = pos.shift(1) * r

# diff = 今仓-昨仓。abs 后 0→1、1→0 都是 1 次换手。持平则 0。
cost = pos.diff().abs() * (fee + slip)

# 给人看、给 Sharpe 用的是净。毛好看、换手高，净会先死。
net = gross - cost

print(pd.DataFrame({"r": r, "pos": pos, "gross": gross, "cost": cost, "net": net}))
print("年成本粗算：换手 × 单边成本 × 2")
