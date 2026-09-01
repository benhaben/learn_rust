"""深度学习单独目录：序列模型照样会泄漏，不是换成 LSTM 就免疫。

运行：python dl/01_seq_leakage.py

# 常见坑

滑动窗口 X[t-19:t] 预测 y[t+1]：窗口右端必须 ≤ t，标签在未来。
若标准化用整段序列的 μ/σ，或把未来 bar 填进窗口，AUC 虚高。
随机打乱窗口（当图像分类）= 金融里的 shuffle=True。

本文件用假序列演示：正确窗口的末元素是 t；错误窗口多看了 t+1。
"""

import numpy as np

rng = np.random.default_rng(13)
# 假日收益。窗口 3 根，预测下一根。
r = rng.normal(0, 0.01, 10)


def windows(r, L, leak):
    # leak=False：X 用 r[i-L:i]，y 用 r[i]（预测「下一根」时 i 是未来，特征停在 i-1）
    # leak=True：X 右端含 r[i]，等于用了标签当天。
    xs, ys = [], []
    for i in range(L, len(r)):
        if leak:
            xs.append(r[i - L + 1 : i + 1])
        else:
            xs.append(r[i - L : i])
        ys.append(r[i])
    return np.array(xs), np.array(ys)


X_ok, y = windows(r, 3, leak=False)
X_bad, _ = windows(r, 3, leak=True)
print("正确窗口最后一格应是 y 的前一根", X_ok[0], "y", y[0])
print("泄漏窗口最后一格等于 y    ", X_bad[0], "y", y[0])
print("口条：LSTM 不免疫泄漏。窗口右端 ≤ t，标签在未来。")
