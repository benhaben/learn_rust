"""进阶：截面 IC = 当天因子与下期收益的相关。

运行：python 12_ic.py
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(4)
# 5 个交易日 × 4 只股票
factor = pd.DataFrame(rng.normal(size=(5, 4)), columns=list("ABCD"))
fwd_ret = factor.shift(0) * 0.1 + rng.normal(scale=0.05, size=factor.shape)
fwd_ret = pd.DataFrame(fwd_ret, index=factor.index, columns=factor.columns)
# 正确：因子_t 对 收益_{t+1}
ic_t = factor.corrwith(fwd_ret.shift(-1), axis=1)
print("每日截面 IC\n", ic_t)
print("IC 均值", float(ic_t.mean()), "标准差", float(ic_t.std()))
