"""绿皮主题：欧式看涨看跌平价。不依赖 BS，只依赖无套利。

运行：python 12_put_call.py

    C - P = S e^{-qT} - K e^{-rT}
无股息：C - P = S - K e^{-rT}（或用远期：C-P = e^{-rT}(F-K)）

口条：合成远期。左边是期权，右边是借钱买标的。哪个贵就卖哪个。
美式有提前行权，平价变不等式。
"""

import numpy as np


def ncdf(x):
    # Φ(x) = (1+erf(x/√2))/2，避免依赖 scipy。
    return 0.5 * (1.0 + np.vectorize(lambda z: __import__("math").erf(z / np.sqrt(2)))(x))


def bs_call_put(s, k, r, sig, t):
    d1 = (np.log(s / k) + (r + 0.5 * sig**2) * t) / (sig * np.sqrt(t))
    d2 = d1 - sig * np.sqrt(t)
    df = np.exp(-r * t)
    c = s * ncdf(d1) - k * df * ncdf(d2)
    p = k * df * ncdf(-d2) - s * ncdf(-d1)
    return c, p


s, k, r, sig, t = 100.0, 100.0, 0.03, 0.2, 1.0
c, p = bs_call_put(s, k, r, sig, t)
fwd = s - k * np.exp(-r * t)
print(f"C={c:.4f} P={p:.4f}  C-P={c-p:.4f}  S-K*exp(-rT)={fwd:.4f}")
print("口条：平价先于模型。BS 只是给 C 和 P 各一个数，差必须锁住。")
