"""进阶：walk-forward；测试前留 embargo。purge 见注释。

运行：python 08_walk_forward.py

# 人话
walk-forward = 用过去训练、下一段当测试，窗口往前滚。模仿实盘：当时只能用已经发生的数据。
标签若看未来 10 天，训练集末尾的 y 已经伸进测试窗 → 和测试共用一段未来，这些点要丢掉。
测试刚结束再封几天不用，避免收益自相关把测试信息带到下一折。

# 目的
交叉验证 / 调参时不偷看未来。只切时间还不够，重叠的标签窗口也要洗。

# 场景
滚动回测、网格搜参、选模型。事件级谁被洗掉、以及准不准 vs 扣费后的钱，见 22。

# 这两个英文为什么是这个意思（AFML 术语）

purge = 清洗、肃清，把脏的清出去（purge cache、名单里删人）。
标签若看未来 10 根，训练集末尾的标签已经伸进测试窗，
和测试集共用一段未来 → 这些点是污染，从训练集洗掉。

embargo = 禁运、封港：这段时间谁也别碰。
测试刚结束，收益可能还自相关、标签窗口可能扫到下一折，
所以再划一段禁区，训练和测试都不用。

    purge    ≈ drop overlapping    洗重叠的脏点
    embargo  ≈ buffer / cooldown   结束后封一段

# 本文件

滚动测试窗，每次训练停在「测试开始 − 标签地平线 − embargo」之前。
事件级谁被洗掉、以及准确率 vs 扣费后的钱，见 22_purge_net.py。
"""

n = 400
embargo = 5
horizon = 10  # 标签看未来 10 根 → 与测试重叠的训练点要 purge
print(f"n={n} embargo={embargo} 标签地平线={horizon}")

for te_end in range(200, n, 40):
    te_st = te_end - 40
    # 测试标签用到 te_end+horizon-1，训练必须在 te_st - horizon 之前结束（purge）
    tr_end = te_st - horizon - embargo
    if tr_end < 50:
        continue
    print(f"训练 [0,{tr_end})  | purge+embargo | 测试 [{te_st},{te_end})")


# n=400 embargo=5 标签地平线=10
# 训练 [0,145)  | purge+embargo | 测试 [160,200)
# 训练 [0,185)  | purge+embargo | 测试 [200,240)
# 训练 [0,225)  | purge+embargo | 测试 [240,280)
# 训练 [0,265)  | purge+embargo | 测试 [280,320)
# 训练 [0,305)  | purge+embargo | 测试 [320,360)
