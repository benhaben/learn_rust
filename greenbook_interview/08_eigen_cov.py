"""绿皮主题：对称矩阵可正交对角化。协方差的特征向量是主风险方向。

运行：python 08_eigen_cov.py

两资产强相关时，最大特征值对应「一起涨」的市场模式，
小特征值对应「多空对冲」残差。PCA 风险、特征组合面试常问这个图。

口条：协方差实对称 ⇒ 特征正交。大特征值 = 解释最多方差的方向。
"""

import numpy as np

rng = np.random.default_rng(8)
# 共同因子 + 一点个性噪声 ⇒ 相关大约 0.8。
f = rng.normal(0, 1, 5_000)
r1 = f + 0.5 * rng.normal(0, 1, 5_000)
r2 = f + 0.5 * rng.normal(0, 1, 5_000)
cov = np.cov(np.vstack([r1, r2]))
w, v = np.linalg.eigh(cov)  # 升序
# 最大特征值的向量。
vmax = v[:, -1]
print("协方差\n", np.round(cov, 3))
print("特征值（小→大）", np.round(w, 3))
print("主方向", np.round(vmax, 3), "  （两资产同号 ≈ 市场）")
print("口条：特征值=该方向方差；特征向量=组合权重（未归一成资金）。")
