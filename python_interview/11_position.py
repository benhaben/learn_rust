"""进阶：信号除以波动再归一，再缩放到目标波动。

运行：python 11_position.py
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(3)
r = pd.DataFrame(rng.normal(0, 0.01, (80, 2)), columns=["A", "B"])
signal = pd.DataFrame({"A": 1.0, "B": -0.5}, index=r.index)
vol = r.rolling(20).std().replace(0, np.nan)
raw = signal / vol
w = raw.div(raw.abs().sum(axis=1), axis=0)
port_r = (w.shift(1) * r).sum(axis=1)
target_vol = 0.10
scale = target_vol / (port_r.std() * np.sqrt(252))
print("未缩放组合年化波动", float(port_r.std() * np.sqrt(252)))
print("缩放系数", float(scale))
print(w.dropna().head())
