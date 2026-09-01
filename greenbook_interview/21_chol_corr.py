"""绿皮后半：相关布朗。先独立正态，再乘 Cholesky，得到指定相关。

运行：python 21_chol_corr.py

    Z = L X，X 独立 N(0,1)，LL^T = ρ 矩阵
MC 多资产、篮子、最差表现期权都靠这个。相关必须是 PSD，否则分解失败。

口条：相关不是两个独立随机数乘 ρ。先分解协方差，再线性组合。
"""

import numpy as np

rng = np.random.default_rng(21)
rho = 0.6
cov = np.array([[1.0, rho], [rho, 1.0]])
l = np.linalg.cholesky(cov)
x = rng.normal(0, 1, size=(2, 30_000))
z = l @ x
emp = np.corrcoef(z)[0, 1]
print("L =\n", np.round(l, 3))
print(f"目标 ρ={rho}  样本相关 {emp:.3f}")
print("口条：Chol(Σ) 把独立正态拧成相关。Σ 必须半正定。")
