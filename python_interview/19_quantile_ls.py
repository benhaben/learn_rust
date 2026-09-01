"""进阶：分位数多空 — IC 之外看「因子大的一组是否真赚」。

运行：python 19_quantile_ls.py

# 人话
按当天因子从大到小切成 5 组（五分位）。做多最高组、做空最低组。
看各组下一期平均收益：若 Q5 > Q4 > … > Q1，才像真因子。中间乱、两头尖，可能是非线性或噪声。

# 目的
IC 是一个相关，会被两只极端票拉动。分层看「因子大的一组是否真赚」。

# 场景
单因子评估、多空组合构建。评估仍要滞后、扣费；本文件只演示截面分层。

# 为什么还要分层

IC 是一个相关，会被两只极端票拉动。
按因子切成 5 组，做多最高组、做空最低组，看各组下期平均收益。
单调：Q5 > Q4 > … > Q1，才像真因子。中间乱、两头尖，可能是非线性或噪声。

评估仍要滞后、扣费。本文件只演示截面分层，不计成本。

# 本文件

5 天 × 20 只股票。因子和「下一期收益」埋了同期斜率（再 shift 成下期）。
每天切五分位，再对天平均。
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(10)
days, n = 5, 20
factor = pd.DataFrame(rng.normal(size=(days, n)))
# 下期收益 ≈ 0.05×今天因子 + 噪声。表上的 fwd 已经是 t+1，和因子同行对齐。
fwd = factor * 0.05 + rng.normal(scale=0.08, size=factor.shape)
fwd = pd.DataFrame(fwd, index=factor.index, columns=factor.columns)

# 每天按因子五分位，看各组 fwd 均值。qcut 标签 1…5。
rows = []
for t in factor.index:
    q = pd.qcut(factor.loc[t], 5, labels=[1, 2, 3, 4, 5])
    mu = fwd.loc[t].groupby(q, observed=False).mean()
    rows.append(mu)
qmean = pd.DataFrame(rows).mean()
print("五组平均下期收益（1=因子最小 … 5=最大）\n", qmean)
print("多空 Q5-Q1", float(qmean.loc[5] - qmean.loc[1]))
print("口条：分层收益要单调；只报 IC 不够。")
