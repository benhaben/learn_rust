"""必考：简单收益用于组合加权；对数收益多期可加。

运行：python 04_returns.py
"""

import numpy as np
import pandas as pd

px = pd.Series([100.0, 101.0, 99.0, 102.0])
R = px.pct_change()
r = np.log(px).diff()
nav = (1 + R.fillna(0)).cumprod()
print("简单收益\n", R)
print("对数收益\n", r)
print("净值\n", nav)
print("R 与 r 接近", np.allclose(r.dropna(), np.log(1 + R.dropna()), atol=1e-12))
