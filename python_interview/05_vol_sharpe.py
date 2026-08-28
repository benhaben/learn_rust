"""必考：年化乘 sqrt(252)；Sharpe / Sortino / 最大回撤。

运行：python 05_vol_sharpe.py
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(0)
r = pd.Series(rng.normal(0.0003, 0.01, 252))
rf_daily = 0.02 / 252

sig_y = r.std(ddof=1) * np.sqrt(252)
ex = r - rf_daily
sharpe = ex.mean() / ex.std(ddof=1) * np.sqrt(252)
down = ex.clip(upper=0)
sortino = ex.mean() / down.std(ddof=1) * np.sqrt(252)
nav = (1 + r).cumprod()
mdd = (nav / nav.cummax() - 1).min()
print(f"年化波动 {sig_y:.2%}")
print(f"Sharpe {sharpe:.2f}  Sortino {sortino:.2f}  MDD {mdd:.2%}")
