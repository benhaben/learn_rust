"""必考：按时间切分；标准化只 fit 训练集。

运行：python 07_leakage_split.py
"""

import numpy as np
import pandas as pd

n = 100
X = pd.DataFrame({"z": np.linspace(0, 1, n)})
cut = int(n * 0.7)
X_tr, X_te = X.iloc[:cut], X.iloc[cut:]
mu, sd = X_tr.mean(), X_tr.std(ddof=0)
X_tr_s = (X_tr - mu) / sd
X_te_s = (X_te - mu) / sd
print("训练折均值应接近 0", float(X_tr_s.mean().iloc[0]))
print("测试折用的是训练折的 mu/sd，均值不必是 0", float(X_te_s.mean().iloc[0]))
print("禁止：sklearn 默认 shuffle=True 的随机切分。")
