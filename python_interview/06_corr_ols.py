"""必考：对收益做相关；预测回归必须滞后因子。

运行：python 06_corr_ols.py
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(1)
mkt = pd.Series(rng.normal(0, 0.01, 80))
y = 0.8 * mkt.shift(1) + rng.normal(0, 0.005, 80)
y = y.dropna()
mkt_lag = mkt.shift(1).reindex(y.index)

# 对收益相关，不要对价格水平
px = (1 + mkt.fillna(0)).cumprod()
print("价格相关（容易虚高）", np.corrcoef(px, px.shift(1).bfill())[0, 1])
print("收益相关", mkt.corr(mkt.shift(1)))

# OLS: y_t = a + b * mkt_{t-1}
X = np.column_stack([np.ones(len(y)), mkt_lag.to_numpy()])
beta, *_ = np.linalg.lstsq(X, y.to_numpy(), rcond=None)
print("intercept, beta_lag", beta)
print("口条：同期回归是风险分解，不是可交易 alpha。")
