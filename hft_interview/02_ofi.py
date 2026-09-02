"""必考：OFI = 买卖档谁在主动。不是「我这单为什么成交」。

运行：python 02_ofi.py

# 人话
买一量增 / 卖一量减 → 偏多主动。反过来偏空。
累加一段，就是这段谁在推盘口。

# 目的
阶段 A 能复现一条可回归的微观特征；别和自己的成交原因混。

# 场景
短窗回归 mid 变化。混进「我的限价成交」，回测会把逆向选择算成 α。
"""

import numpy as np
import pandas as pd

# 四次簿记：买一量、卖一量。价不变，只看量变（示意）。
bid = pd.Series([10, 14, 14, 9], name="bid_sz")
ask = pd.Series([12, 12, 8, 8], name="ask_sz")
# 买增为正、卖增为负（卖档变厚 = 供给增加）。
ofi = bid.diff().fillna(0) - ask.diff().fillna(0)
print(pd.DataFrame({"bid": bid, "ask": ask, "ofi": ofi}))
print("累计 OFI", float(ofi.sum()), "  （前两步偏多，第三步卖档变薄也偏多，第四步买档被砸）")
print("口条：OFI 问现在谁在主动。你这单成交是另一问。")
