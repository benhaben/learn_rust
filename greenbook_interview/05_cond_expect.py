"""绿皮主题：条件期望。E[E[X|Y]] = E[X]（塔性质）。

运行：python 05_cond_expect.py

先按 Y 分组看组内均值，再按 Y 的分布平均回去，等于不管 Y 直接算 E[X]。
量化：先算「这个波动档位下的期望收益」，再按档位权重加权，等于无条件均值。

口条：条件期望是随机变量（是 Y 的函数）；再取期望把条件抹掉。
"""

import numpy as np

rng = np.random.default_rng(5)
# Y=档位 0/1/2；X|Y ~ N(档位, 1)，所以无条件均值应是 E[Y]=1。
y = rng.integers(0, 3, 30_000)
x = y + rng.normal(0, 1, 30_000)

# 组内均值 = E[X|Y=k] 的估计。
inner = np.array([x[y == k].mean() for k in (0, 1, 2)])
# 塔：用 Y 的频率加权。
p_y = np.array([(y == k).mean() for k in (0, 1, 2)])
tower = inner @ p_y

print(f"E[X]={x.mean():.3f}  塔性质 {tower:.3f}  组内均值 {inner.round(3)}")
print("口条：E[E[X|Y]]=E[X]。条件期望是 Y 的函数，不是一个数。")
