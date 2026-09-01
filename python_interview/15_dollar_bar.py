"""进阶：dollar bar 在成交额凑满时收一根，不是均匀时钟。

运行：python 15_dollar_bar.py

# 为什么不用均匀时间 bar

1 分钟里有时 2 笔、有时 200 笔。时钟 bar：信息少的时段也占一根，
信息炸的时段把很多信息捏成一根。统计量（波动、相关）被时钟扭曲。

AFML：按信息量切。
    tick bar     每 N 笔
    volume bar   每 N 股
    dollar bar   每 N 元成交额（价×量累加）

活跃时 dollar bar 出得更勤，安静时更稀。更接近「信息到达」。

# 本文件

假 tick：价格随机走，量有时大有时小。
同样一段行情：1 分钟时钟 bar 固定根数；dollar bar 在放量时更多。
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(6)
n = 400
# 假 tick：价、量。后 100 笔放量（模拟一段热闹）。
px = 100 * np.exp(np.cumsum(rng.normal(0, 0.0003, n)))
sz = rng.integers(1, 10, n).astype(float)
sz[300:] *= 8
dollar = px * sz

# 时钟：每 20 笔当「1 分钟」（均匀）。热闹段和安静段根数一样多。
time_bars = n // 20

# dollar bar：成交额凑满阈值就收一根。阈值 = 全样本额 / 目标大约 20 根。
thresh = dollar.sum() / 20
bars, acc = 0, 0.0
in_busy = 0
for i, d in enumerate(dollar):
    acc += d
    if acc >= thresh:
        bars += 1
        if i >= 300:
            in_busy += 1
        acc = 0.0

print(f"时钟 bar（每 20 笔）{time_bars} 根，热闹段也按同一节奏切")
print(f"dollar bar 共 {bars} 根，其中热闹段（后 100 笔）{in_busy} 根")
print("口条：时钟均匀，信息不均匀；dollar bar 跟着成交额走。")
