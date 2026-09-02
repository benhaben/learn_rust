"""进阶：截面 IC = 当天因子与下期收益的相关。

运行：python 12_ic.py

# 人话
截面 IC = 同一天横着看：这几只股票的因子值 vs 它们下一期的收益，算一个相关。
正 IC：因子大的票，下一期更容易涨。日 IC 往往只有 0.02～0.05，也能用。
Pearson 看原始数值的直线关系；Spearman 看名次的同向关系。
Spearman 不被「数值有多极端」拽歪，但仍会被「谁占住两头名次」抬高。
数字例子和「为什么还要分层」见 19。

# 目的
先检验因子有没有预测力，再谈分层（19）、回测。禁止因子对同期收益（那是解释不是预测）。

# 场景
单因子检验、因子库筛选。IR ≈ IC均值 / IC标准差（再按天数或广度开根，口径要说清）。

# IC 是什么

截面 IC：同一天，横着看 4 只股票，因子值 vs 下期收益，算一个相关。
正 IC：因子大的票，下一期更容易涨。日 IC 往往很小（0.02～0.05 也能用），
IR ≈ IC均值 / IC标准差（再按有效天数或广度开根，口径要说清）。

禁止：因子_t 对 同期收益_t。那是解释不是预测（和 06 同期回归同一类错）。

# 本文件造数的瑕疵

fwd_ret = 0.1 * 当天因子 + 噪声，埋的是同期关系。
IC 用的是 fwd_ret.shift(-1)，看的是下一根。只有 4 只票，相关很吵，
种子 4 仍可能打出 0.6 这种一天，最后一天没有下一根，IC 是 NaN。
"""

import numpy as np
import pandas as pd

# 种子 4，可复现。
rng = np.random.default_rng(4)

# 5 行（交易日）× 4 列（股票 A–D）。normal() 默认均值 0、标准差 1。
# list("ABCD") = ["A","B","C","D"]。每格一个因子值（分数，不是收益）。
factor = pd.DataFrame(rng.normal(size=(5, 4)), columns=list("ABCD"))

# 人造「远期收益」：0.1×因子 + N(0, 0.05)。shift(0) 什么都不做，等于当天因子。
# 所以埋的是同期：收益_t ≈ 0.1×因子_t。size=和因子同一形状。
fwd_ret = factor.shift(0) * 0.1 + rng.normal(scale=0.05, size=factor.shape)

# 上一行可能是 ndarray，包回 DataFrame，行列标签和因子对齐。
fwd_ret = pd.DataFrame(fwd_ret, index=factor.index, columns=factor.columns)

# corrwith(..., axis=1)：每一行横着，4 只股票算一次相关 → 每天一个 IC。
# shift(-1)：用明天的收益。最后一行没有明天，IC 是 NaN。
# 正确口条：因子_t 对 收益_{t+1}。不要对 fwd_ret 本身（同期）。
ic_t = factor.corrwith(fwd_ret.shift(-1), axis=1)

print("每日截面 IC\n", ic_t)
# 4 天有效 IC（第 5 天 NaN 被 mean/std 跳过）。样本极短，均值会晃。
print("IC 均值", float(ic_t.mean()), "标准差", float(ic_t.std()))
