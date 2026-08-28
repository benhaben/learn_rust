"""压轴：信号用 t 的信息，收益至少用下一根。

运行：python 13_execution_lag.py
"""

import pandas as pd

r = pd.Series([0.01, -0.02, 0.015, 0.00], name="r")
signal = pd.Series([1, 1, -1, 0], name="signal")  # 收盘后才知道
# 收盘算信号、同一根收盘成交 = 偷看
cheat = signal * r
# 保守：今天信号，明天收益
ok = signal.shift(1) * r
print(pd.DataFrame({"r": r, "signal": signal, "cheat": cheat, "ok": ok}))
print("口条：信号 t，成交至少 t+1。PIT 成分/财报用 merge_asof。")
