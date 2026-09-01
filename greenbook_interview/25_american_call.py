"""绿皮后半：无股息欧式看涨 = 美式看涨。提前行权不划算。

运行：python 25_american_call.py

行权得到 S−K，但丢掉保险（还能再涨）和时间价值。
无股息：持有标的没有「错过分红」；钱放在 K 上还能吃利息。
有股息：除息前可能行权。美式看跌即使无股息也可能提前（下跌有界、利率）。

口条：无股息美式看涨不当场行权。看跌、有股息，另说。
"""

import math
import numpy as np


def ncdf(x):
    return 0.5 * (1.0 + math.erf(x / math.sqrt(2.0)))


def bs_call(s, k, r, sig, t):
    d1 = (math.log(s / k) + (r + 0.5 * sig**2) * t) / (sig * math.sqrt(t))
    d2 = d1 - sig * math.sqrt(t)
    return s * ncdf(d1) - k * math.exp(-r * t) * ncdf(d2)


s, k, r, sig, t = 110.0, 100.0, 0.05, 0.2, 1.0
c = bs_call(s, k, r, sig, t)
exercise = s - k
# 欧式下界 S−Ke^{-rT}（平价+P≥0）。
lower = s - k * math.exp(-r * t)
print(f"欧式 C={c:.3f}  当场行权 {exercise:.3f}  下界 S-Ke^{{-rT}}={lower:.3f}")
print(f"C > 行权  差 {c - exercise:.3f}  （时间价值，丢掉就亏）")
print("口条：无股息，C ≥ S−Ke^{-rT} > S−K。美式多一个权利，但不该用。")
