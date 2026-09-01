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
不是「趋势里更准」，只换采样。训练/信号可以用，须和实盘同一套 bar。

# 本文件

假 tick：价格随机走，后 100 笔放量。
时钟按每 20 笔切，根数固定；dollar bar 会扎堆出现在放量段。
"""

import numpy as np
import pandas as pd  # 本文件没用到，和其他脚本统一留着

# 固定种子，每次同一条 400 笔假 tick（不是 400 根分钟线）。
rng = np.random.default_rng(6)
n = 400

# 假成交价：对数收益累加再 exp，起点 100。σ 很小，对比主要靠「量」不靠暴涨。
px = 100 * np.exp(np.cumsum(rng.normal(0, 0.0003, n)))

# 每笔 1～9「股」（integers 上界不含 10）。转 float 才能 *= 8。
sz = rng.integers(1, 10, n).astype(float)
# 下标 300…399：最后 100 笔量乘 8 = 热闹段。笔数仍是 100，额大约大 8 倍。
sz[300:] *= 8

# 逐笔成交额。dollar bar 累加这个；volume bar 才累加 sz。
dollar = px * sz

# 假装「每 20 笔 = 1 分钟」。400//20=20 根，热闹/安静同一节奏。
# 真 1 分钟按墙钟；这里不引进时间戳，只用均匀笔数代替时钟。
time_bars = n // 20

# 阈值 = 全样本总额 / 20，好和时钟 20 根比。
# 演示可以；研究里阈值应事先定或只用过去估，否则用未来总额定尺子。
thresh = dollar.sum() / 20

# acc=当前这根还没凑满的额；bars=已收几根；in_busy=在放量段收的根数。
bars, acc = 0, 0.0
in_busy = 0

# i=第几笔 tick（0…399），d=这笔的价×量。
for i, d in enumerate(dollar):
    acc += d
    if acc >= thresh:
        bars += 1
        # 收棒时已经走进后 100 笔，记到热闹段。
        if i >= 300:
            in_busy += 1
        # 清空开下一根。溢出部分丢掉，没结转（生产常 acc -= thresh）。
        acc = 0.0

# 循环结束 acc 里可能还剩不到一根，所以常是 19 根不是 20。

print(f"时钟 bar（每 20 笔）{time_bars} 根，热闹段也按同一节奏切")
print(f"dollar bar 共 {bars} 根，其中热闹段（后 100 笔）{in_busy} 根")
print("口条：时钟均匀，信息不均匀；dollar bar 跟着成交额走。")
