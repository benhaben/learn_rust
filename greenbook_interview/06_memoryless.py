"""绿皮主题：无记忆。指数 / 几何：已经等了 t，再等 s 的分布和从头等 s 一样。

运行：python 06_memoryless.py

    P(T>t+s | T>t) = P(T>s)

公交「已经 10 分钟没来，所以马上会来」——若到达是指数，这个直觉是错的。
正态、均匀都有记忆；绿皮常拿指数和几何开刀。

口条：无记忆 ⇒ 等过的时间不改变剩余寿命的分布。
"""

import numpy as np

rng = np.random.default_rng(6)
# 指数，均值 10（尺度=10）。
t = rng.exponential(10, 80_000)
# 已经活过 5：剩余寿命。
remain = t[t > 5] - 5

print(f"无条件 P(T>3)={np.mean(t > 3):.3f}")
print(f"已过 5 再等 3  P(剩余>3)={np.mean(remain > 3):.3f}  （应接近）")
print(f"剩余均值 {remain.mean():.2f}  仍应≈10，不是 5")
print("口条：指数/几何无记忆；「等很久了所以快来了」要另给分布。")
