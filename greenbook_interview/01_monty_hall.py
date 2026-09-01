"""绿皮主题：信息会改概率。三扇门，主持人开一只山羊，换不换？

运行：python 01_monty_hall.py

先猜对的概率是 1/3。主持人打开另一扇山羊之后，剩下那扇是车的概率是 2/3。
不是「还剩两扇所以 1/2」——主持人不是随机开门，他的选择带信息。

口条：换门赢 2/3。贝叶斯：似然把概率从先验里重新分配。
"""

import numpy as np

rng = np.random.default_rng(1)
n = 20_000
# 车在 0/1/2，我始终先挑 0。
prize = rng.integers(0, 3, n)
stay = prize == 0

# 主持人开一只山羊：不能开我的门，不能开车。
host = np.empty(n, dtype=int)
for i in range(n):
    goats = [d for d in (1, 2) if d != prize[i]]
    host[i] = goats[0] if len(goats) == 1 else rng.choice(goats)

# 换到「既不是我的、也不是主持人开的」那扇。
switch_door = 3 - 0 - host
switch = prize == switch_door

print(f"坚持 {stay.mean():.3f}（应≈1/3）  换门 {switch.mean():.3f}（应≈2/3）")
print("口条：主持人开门是信息，不是随机剩两扇各 1/2。")
