"""进阶：把 14 的标签接到 08 的切分和 10 的扣费。

运行：python 22_purge_net.py

# 08 / 10 已经有了，缺的是「接在一起」

08 只印训练/测试下标，看不见哪几条三重屏障样本被洗掉。
10 只演示昨仓×今收益 − |Δ仓|×成本，没有屏障事件。
本文件仍用假价格（和 14 一样可复现），练的是实盘**流程**，不是某只股票。
真实行情上场时，换 px 来源即可，切分和扣费规则不变。

# purge / embargo 在事件上长什么样

事件 t 的 y 用到价格 t+1 … t+bars_to_hit。
测试从 te_st 开始：若训练事件 t < te_st，但 t+bars_to_hit >= te_st，
这段未来已经伸进测试窗 → 从训练集洗掉（purge）。
embargo：测试结束后再封 embargo 根，下一折训练不要立刻去贴测试尾，
避免收益自相关把测试信息带到下一折。

# 评估看扣费后的钱

「永远预测 +1」会有一个准确率。
同一批测试事件若都做多、拿到撞障/超时再平，扣双边成本，
净收益经常是负的——准确率不对称，利润在尾部。
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(5)
px = pd.Series(100 * np.exp(np.cumsum(rng.normal(0, 0.01, 140))))

vol_win = 20
max_h = 10
# 单边 10bp；开和平各付一次，所以一笔事件扣 2×。
fee_slip = 0.0010
embargo = 5
# 一折：测试事件入场落在 [te_st, te_end)。
te_st, te_end = 80, 105

vol = np.log(px).diff().rolling(vol_win).std()
pt = sl = 1.5 * vol

rows = []
for t in range(vol_win, len(px) - max_h):
    p0 = px.iloc[t]
    up, dn = p0 * (1 + pt.iloc[t]), p0 * (1 - sl.iloc[t])
    path = px.iloc[t + 1 : t + 1 + max_h]
    lab, hit = 0, max_h
    for i, p in enumerate(path, start=1):
        if p >= up:
            lab, hit = 1, i
            break
        if p <= dn:
            lab, hit = -1, i
            break
    # 平仓价：撞上的那根，或超时最后一根。做多的简单收益。
    p_exit = float(path.iloc[hit - 1])
    ret = p_exit / float(p0) - 1.0
    # 标签用到的最后一根下标。>= te_st 就和测试窗抢未来。
    last = t + hit
    net = ret - 2 * fee_slip
    rows.append((t, lab, hit, last, ret, net))

ev = pd.DataFrame(rows, columns=["t", "label", "hit", "last", "ret", "net"])

# 入场在测试窗内 = 本折测试事件。
test = ev[(ev["t"] >= te_st) & (ev["t"] < te_end)]
# 入场在测试前 = 候选训练；其中 last 伸进测试的要 purge。
train_raw = ev[ev["t"] < te_st]
overlap = train_raw["last"] >= te_st
purged = train_raw[overlap]
train = train_raw[~overlap]
# 测试结束后封禁：这些事件本折不用，也不准立刻当下一折训练。
blocked = ev[(ev["t"] >= te_end) & (ev["t"] < te_end + embargo)]

print(f"测试入场 [{te_st},{te_end})  purge: last>=te_st  embargo={embargo} 根")
print(f"候选训练 {len(train_raw)}  洗掉 {len(purged)}  干净训练 {len(train)}")
print(f"测试事件 {len(test)}  封禁 {len(blocked)}  条（t 在 [{te_end},{te_end + embargo})）")
print("被 purge 的训练事件（y 伸进测试窗）\n", purged[["t", "label", "hit", "last"]].head(8))

# 永远猜 +1：只看标签准不准，不看钱。
acc = float((test["label"] == 1).mean()) if len(test) else float("nan")
# 同一批事件都做多、拿到退出，扣双边成本。
net_sum = float(test["net"].sum())
net_mean = float(test["net"].mean()) if len(test) else float("nan")
print(f"\n测试上「永远预测 +1」准确率 {acc:.1%}")
print(f"同一批都做多、扣 2×{fee_slip:.2%} 后：合计净 {net_sum:.4f}  每笔均净 {net_mean:.4f}")
print("口条：purge 洗重叠标签；embargo 封测试后一段。评估看净，不看 accuracy。")
