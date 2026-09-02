"""必考：HJB 留下的两句。货多往下挪；怕风险、波动大则价差宽。

运行：python 05_inventory_spread.py

# 人话
不做完整 HJB。Avellaneda–Stoikov 口条：
  保留价 = mid − 库存 × γσ²(T−t)
  价差随 γ、σ 变宽
多头（库存>0）保留价低于 mid → 整张往下，急着卖。

# 目的
能在白板上写偏置和价差，不推 PDE。

# 场景
做市报价。PCM 槽位、只禁开、到期日：都是「未来可行集」进状态，同一类控制。
"""

import numpy as np

mid, gamma, sig, tau = 100.0, 0.5, 0.25, 1.0  # 数字放大，方便看方向
print("库存   保留价    半价差(示意 γσ√τ)")
for inv in (-3, 0, 3, 8):
    reserve = mid - inv * gamma * sig**2 * tau
    # 价差随风险厌恶和波动变宽（示意）。
    half = 0.05 + 0.5 * gamma * sig * np.sqrt(tau)
    print(f"{inv:4d}   {reserve:.4f}   {half:.4f}")
print("口条：货多保留价下移；γ、σ 大则价差宽。不管涨跌预测。")
