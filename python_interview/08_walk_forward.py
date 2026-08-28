"""进阶：walk-forward；测试前留 embargo。purge 见注释。

purge = 从训练集删掉和测试标签时间重叠的点。
embargo = 测试结束后再封一段，谁也别用。

运行：python 08_walk_forward.py
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
