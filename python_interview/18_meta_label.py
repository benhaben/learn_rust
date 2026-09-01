"""进阶：meta-labeling — 主模型定方向，次模型决定下不下。

运行：python 18_meta_label.py

# 人话
主模型（或规则）只输出方向：+1 / −1（看多还是看空）。
次模型回答「主模型这次会不会对」：下 / 不下（或仓位大小）。
对了才交易 → 次数变少，精确率往往升，召回会掉（错过一些对的）。

# 目的
已有方向时，过滤没把握、拥挤、高成本的时段，降换手。

# 场景
趋势规则 + 分类器决定做不做；三重屏障打完 +1/−1 后再预测会不会撞对的边。
本文件用「|昨收益| 大于中位数」当假次模型，真 meta 要另训一个分类器。

# 两个模型

主模型（或规则）：只输出边，+1 / -1（今天看多还是看空）。
次模型：分类「主模型这次会不会对」。输出下 / 不下（或仓位大小）。

对了才交易 → 换手下降，精确率上升。召回会掉（错过一些对的）。
适合：已有方向（趋势、套利边），要过滤拥挤/高成本时段。

# 和三重屏障

AFML 常：屏障给出 +1/-1/0 之后，meta 预测「会不会碰到上/下障」。
本文件简化：主信号 = 昨收益符号（动量），对 = 今收益与昨仓同号。
次过滤：只在 |昨收益| 大于中位数时下（当「有把握」）。

# 本文件

对比「主信号每天都做」和「meta 过滤后再做」的次数与同向次数。
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(9)
r = pd.Series(rng.normal(0, 0.01, 200))
# 主边：昨涨则今做多（收盘后才有昨收益，乘今 r 不偷看）。
side = np.sign(r.shift(1))
correct = (side * r) > 0

# 次过滤：|昨收益| 大才做。演示用；真 meta 是用特征训一个分类器。
confident = r.shift(1).abs() > r.shift(1).abs().median()
trade = side.where(confident, 0.0)

n_all = int(side.fillna(0).ne(0).sum())
n_meta = int(trade.ne(0).sum())
hit_all = float(correct[side.ne(0)].mean())
hit_meta = float(((trade * r) > 0)[trade.ne(0)].mean())
print(f"主模型每天做：{n_all} 次，同向率 {hit_all:.2%}")
print(f"meta 过滤后：{n_meta} 次，同向率 {hit_meta:.2%}")
print("这份假数据过滤不抬准确率（过滤特征没信息），但次数少了。真 meta 要能预测「对不对」。")
print("口条：主模型定方向，次模型定下不下。换手少、精确率往往升。")
