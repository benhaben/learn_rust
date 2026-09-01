"""绿皮主题：贝叶斯。罕见病 + 高准确检测，阳性仍可能没事。

运行：python 03_bayes_test.py

    P(有病|阳) = P(阳|有病) P(有病) / P(阳)

P(阳) = 真阳 + 假阳。患病率 1%、灵敏度 99%、误报 1% 时，
阳性后有病大约 50%，不是 99%。

口条：先写先验，再乘似然。量化里：信号「很准」但事件很稀，后验仍薄。
"""

import numpy as np

rng = np.random.default_rng(3)
n = 200_000
p_disease = 0.01
sens, fpr = 0.99, 0.01  # 有病检出 / 没病误报

sick = rng.random(n) < p_disease
# 有病按灵敏度出阳；没病按误报率出阳。
positive = np.where(sick, rng.random(n) < sens, rng.random(n) < fpr)

# 条件：在阳性子样本里，真正有病的比例。
post = sick[positive].mean()
theory = (sens * p_disease) / (sens * p_disease + fpr * (1 - p_disease))
print(f"模拟 P(有病|阳)={post:.3f}  公式 {theory:.3f}")
print("口条：准的是似然，后验还要除以「阳得很常见」。")
