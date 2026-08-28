"""进阶：平稳特征；朴素标签 = 下一根收益。

运行：python 09_features_label.py
"""

import numpy as np
import pandas as pd

px = pd.Series(100 * np.exp(np.cumsum(np.random.default_rng(2).normal(0, 0.01, 60))))
feat = pd.DataFrame(index=px.index)
feat["r1"] = np.log(px).diff()
feat["vol20"] = feat["r1"].rolling(20).std()
feat["z"] = feat["r1"] / feat["vol20"]
y = feat["r1"].shift(-1)  # 下一根；特征必须停在 t
print(feat.tail(3))
print("标签 y 比特征多看未来 1 根，成交至少再晚 1 根（见 13）。")
# 不要 feat["close"] 直接进树 —— 模型会学价格水平
