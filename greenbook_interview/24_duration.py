"""绿皮后半：久期 / DV01。债券价格对收益率的一阶。

运行：python 24_duration.py

修正久期 D_mod ≈ −(1/P) dP/dy
DV01 ≈ P × D_mod × 0.0001   （收益率 1bp 价格变多少钱）
凸性是二阶：收益率大动时，久期低估涨、高估跌（价格凸）。

口条：久期 = 加权平均到期（麦考利）再 / (1+y) 才是价格敏感。先说 y 的计息。
"""

import numpy as np

# 每年付 5 的 3 年债，面值 100，YTM=4% 年复。
cfs = np.array([5.0, 5.0, 105.0])
times = np.array([1.0, 2.0, 3.0])
y = 0.04
dfs = (1 + y) ** (-times)
p = float(cfs @ dfs)
# 麦考利：现金流时间按现值加权。
mac = float((times * cfs * dfs).sum() / p)
mod = mac / (1 + y)
dv01 = p * mod * 1e-4
# 数值：y 上下 1bp。
p_up = float(cfs @ (1 + y + 1e-4) ** (-times))
p_dn = float(cfs @ (1 + y - 1e-4) ** (-times))
print(f"P={p:.4f}  麦考利 {mac:.3f}  修正 {mod:.3f}  DV01 {dv01:.4f}")
print(f"y+1bp 价格变 {p_up - p:.4f}  （约 −DV01）")
print("口条：修正久期管一阶；大波动还要凸性。DV01 是 1bp 的钱。")
