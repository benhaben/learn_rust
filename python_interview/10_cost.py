"""必考：仓位必须滞后；净收益扣换手成本。

运行：python 10_cost.py
"""

import numpy as np
import pandas as pd

r = pd.Series([0.01, -0.01, 0.02, 0.00])
pos = pd.Series([0.0, 1.0, 1.0, 0.0])  # 当日想要的仓
fee, slip = 0.0005, 0.0005
gross = pos.shift(1) * r
cost = pos.diff().abs() * (fee + slip)
net = gross - cost
print(pd.DataFrame({"r": r, "pos": pos, "gross": gross, "cost": cost, "net": net}))
print("年成本粗算：换手 × 单边成本 × 2")
