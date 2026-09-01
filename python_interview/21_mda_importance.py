"""进阶：MDA 置换重要性；树的 MDI 在金融里会骗人。

运行：python 21_mda_importance.py

# MDI vs MDA

MDI（Mean Decrease Impurity）：树在训练里「分裂时用了谁」。
相关特征会抢分裂，噪声列也可能分到一点。样本内、会泄漏。

MDA（Mean Decrease Accuracy）：时间外，把某一列打乱，看分数掉多少。
掉得多 = 真有用。必须在测试折上做，打乱只打那一列、对齐时间。

SFI：一次只用一个因子，避免共线抢功。

# 本文件

y = 0.8×x_true + 噪声。x_noise 与 y 无关。
「模型」就是 x 和 y 的相关（当分数）。
打乱 x_true，相关应垮；打乱 x_noise，相关几乎不动。
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(12)
n = 80
x_true = pd.Series(rng.normal(size=n))
x_noise = pd.Series(rng.normal(size=n))
y = 0.8 * x_true + rng.normal(scale=0.4, size=n)


def score(x, y):
    return float(pd.Series(x).corr(pd.Series(y)))


base = score(x_true, y)
# 置换：打乱这一列与 y 的配对，时间顺序乱了。
drop_true = base - score(rng.permutation(x_true.to_numpy()), y)
drop_noise = score(x_noise, y) - score(rng.permutation(x_noise.to_numpy()), y)
print("x_true 与 y 相关", base)
print("打乱 x_true 后相关掉了", drop_true, "（应明显）")
print("打乱 x_noise 后相关掉了", drop_noise, "（应接近 0）")
print("口条：重要性在时间外置换；别信训练里的 MDI。")
