"""绿皮主题：有限资本的随机游走会被边界吸收（赌徒破产）。

运行：python 09_gambler_ruin.py

公平游戏 p=1/2，从 i 出发，目标 N，破产在 0：
    P(先到 N) = i/N
不公平 p<1/2 时，即使用很小的劣势，N 大时几乎必破产。

口条：公平是线性；劣势是指数衰减。风控：时间够长，有限保证金会被吃光。
"""

import numpy as np

rng = np.random.default_rng(9)


def ruin_hit(i, n, p, trials=8000, cap=20_000):
    """从 i 出发，0 破产 / n 赢，返回打到 n 的频率。"""
    x = np.full(trials, i)
    alive = np.ones(trials, dtype=bool)
    win = np.zeros(trials, dtype=bool)
    for _ in range(cap):
        if not alive.any():
            break
        step = np.where(rng.random(trials) < p, 1, -1)
        x = np.where(alive, x + step, x)
        hit_n = alive & (x >= n)
        hit_0 = alive & (x <= 0)
        win |= hit_n
        alive &= ~(hit_n | hit_0)
    return float(win.mean())


i, n = 10, 50
fair = ruin_hit(i, n, 0.50)
edge = ruin_hit(i, n, 0.48)
print(f"公平 p=0.5  从 {i} 到 {n}：模拟 {fair:.3f}  公式 {i/n:.3f}")
print(f"劣势 p=0.48 同一目标：模拟 {edge:.3f}  （远低于 i/N）")
print("口条：公平 P=i/N；负期望再大的本金也会被时间干掉。")
