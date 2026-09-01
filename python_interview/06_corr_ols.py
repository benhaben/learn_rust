"""必考：对收益做相关；预测回归必须滞后因子。

运行：python 06_corr_ols.py

# 人话
相关 = 两列数「一起动」的程度（−1 到 1）。价格都在爬，相关会假高，要对收益。
回归 y ≈ a + b x：同期 x 只能解释「今天一起涨」（风险分解），
要预测必须用昨天的 x（滞后），否则收盘才知道的数吃了当天涨跌。

# 目的
避免把趋势当预测力；分清「说明书上的 beta」和「开盘前能用的 alpha」。

# 场景
股票/因子相关矩阵、估市场 beta、预测回归。禁止 px.corr(昨 px) 当选股依据。

# 为什么相关要对收益，不要对价格

价格是随机游走滚出来的：今天 ≈ 昨天 × (1+r)。
px 和 px.shift(1) 几乎是同一条慢慢爬的线，相关会接近 1，
这叫虚假相关：两边都有趋势，不是「今天能预测明天」。
两只都在牛市里涨的股票，价格相关也会虚高。

收益已经差分过（去掉了水平），相关才表示「一起动多少」。
危机里收益相关会冲向 1，分散失效——那是真的同涨同跌，不是趋势假象。

# 为什么预测必须用昨天的因子

OLS：找 a、b，让 y ≈ a + b x 的平方误差最小。

    y_t = a + b * mkt_t + e     同期：风险分解（这只股票的 beta）
    y_t = a + b * mkt_{t-1} + e 滞后：才谈得上预测

同期回归：用「今天市场涨了」解释「今天股票涨了」。收盘后你才知道 mkt_t，
不能据此在今天开盘买。残差是事后说明书（风险分解：市场暴露 vs 特异），
不是开盘前就能用的信号，所以不是可交易 alpha。
要预测，特征必须停在 t 之前：用 mkt_{t-1} 对 y_t。

本文件造的 y 就是 0.8 * 昨市场 + 噪声，所以估出来的 b 应靠近 0.8。

# 本文件在做什么

造 80 天市场收益 → 用昨市场生成 y → 对比价格相关 vs 收益相关
→ 用 [1, mkt_{t-1}] 做最小二乘，打印截距和滞后 beta。
"""

import numpy as np
import pandas as pd

# rng = random number generator。default_rng(种子) 造一台可复现的骰子。
# 种子 1，和 05 的 0 错开；同一文件每次抽出同一串数，方便对答案。
rng = np.random.default_rng(1)

# rng.normal(均值, 标准差, 个数)：正态抽 80 个数，不是「最小值、最大值」。
# 0 = 平均不涨不跌，0.01 = 日波约 1%（多数落在 ±2σ，没有硬边界）。
# 返回 ndarray。套 Series 是为了后面 shift / corr / 按 index 对齐，不是正态必须用 pandas。
mkt = pd.Series(rng.normal(0, 0.01, 80))

# shift(1)：整列往「更晚」挪一格（横着=往右，竖着=往下），头一格 NaN。
# 所以 y[1] 用的是 mkt[0]。shift(-1) 会把未来填进今天，那是泄漏。
# 噪声 σ=0.005，比市场更瘦。第 0 格没有昨天，y[0] 是 NaN。
y = 0.8 * mkt.shift(1) + rng.normal(0, 0.005, 80)

# lstsq 吃不下 NaN。丢掉缺历史的第 0 行，y 从下标 1 起，长度 79。
y = y.dropna()

# 按 y 剩下的标签取昨市场，两边同一批日期：y[1] 对 mkt[0]。
# 只对两个长度不同的数组硬做会对错行。
mkt_lag = mkt.shift(1).reindex(y.index)

# 市场收益连乘滚成「价格」。fillna(0)：第一天当 0 收益，价格从 1 起。
px = (1 + mkt.fillna(0)).cumprod()

# 反面教材：价格 vs 昨价格。两条线几乎贴在一起，相关接近 1，不是预测力。
# corrcoef 按「好多列变量」设计，返回相关矩阵，不直接给一个 ρ：
#   [[1, ρ], [ρ, 1]]  对角是自己和自己，永远 1；ρ 在非对角 [0,1]（或 [1,0]）。
# shift 开头是 NaN；corrcoef 要等长无 NaN，bfill 用后值填第一格才能算。
# 只要一对 Series，用 px.corr(px.shift(1)) 直接得标量（自动跳过 NaN）。
print("价格相关（容易虚高）", np.corrcoef(px, px.shift(1).bfill())[0, 1])

# pandas .corr() 就是一个数。市场是白噪声，今天收益 vs 昨天收益应靠近 0。
print("收益相关", mkt.corr(mkt.shift(1)))

# lstsq = least squares（最小二乘）；linalg = linear algebra。
# 找 β 让 ‖y − Xβ‖² 最小，不必自己写 (XᵀX)⁻¹ Xᵀy。
# y ≈ a + b×昨市场 写成 y ≈ X @ [a, b]：
#   第一列全 1 → 乘 a（截距）。没有这列，直线被逼过原点。
#   第二列昨市场 → 乘 b。column_stack 两列并排。
# to_numpy() 脱掉标签，lstsq 只吃纯数组。
X = np.column_stack([np.ones(len(y)), mkt_lag.to_numpy()])
beta, *_ = np.linalg.lstsq(X, y.to_numpy(), rcond=None)

# beta 和 X 的列一一对应，不是带列名的表。
# beta[0] 截距：造 y 时没加常数，噪声均值 0，应靠近 0（如 -6e-4）。
# beta[1] 斜率：埋的是 0.8；样本短有噪声，会在附近晃（如 0.88），不必恰好相等。
print("intercept, beta_lag", beta)
# 同期 = 收盘后拆账；alpha = 开盘前就能用的预测。残差高 ≠ 能赚钱。
print("口条：同期回归是风险分解，不是可交易 alpha。")
