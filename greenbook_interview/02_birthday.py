"""绿皮主题：碰撞比直觉快。23 人同一天生日的概率已过一半。

运行：python 02_birthday.py

365 天，k 人，至少两人同日：
    1 - 365! / ((365-k)! / 365^k)   （近似）
不是「365/2 才 50%」。配对数量是 C(k,2)，增长是平方的。

口条：哈希碰撞、交易日撞单，同一思路。23 ≈ 50%，55 ≈ 99%。
"""

import numpy as np

rng = np.random.default_rng(2)


def collide_prob(k, days=365, trials=8000):
    bdays = rng.integers(0, days, size=(trials, k))
    # 每行 unique 长度 < k ⇒ 有重复。
    return np.mean([len(np.unique(row)) < k for row in bdays])


for k in (10, 23, 40, 55):
    print(f"{k:2d} 人  模拟 P(至少一对)={collide_prob(k):.3f}")
print("口条：配对是 k(k-1)/2，不是 k/365。")
