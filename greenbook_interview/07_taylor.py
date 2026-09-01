"""绿皮主题：泰勒。把光滑函数在一点附近换成多项式。

运行：python 07_taylor.py

    e^x ≈ 1 + x + x²/2 + x³/6 + …

离展开点近，低阶就够；离得远，再多项也会炸。面试用来：
小收益 log(1+r)≈r、BS 推导里的展开、久期是价格对 y 的一阶。

口条：泰勒是本地近似。先说在哪一点展开、留到几阶。
"""

import numpy as np

xs = np.array([0.1, 0.5, 1.5])
exact = np.exp(xs)
# 0～3 阶。
p0 = np.ones_like(xs)
p1 = p0 + xs
p2 = p1 + xs**2 / 2
p3 = p2 + xs**3 / 6

print("x      真值     1阶      2阶      3阶")
for i, x in enumerate(xs):
    print(f"{x:.1f}  {exact[i]:.4f}  {p1[i]:.4f}  {p2[i]:.4f}  {p3[i]:.4f}")
print("口条：0.1 时 1 阶就够；1.5 时 3 阶还差一点。先定点再谈阶。")
