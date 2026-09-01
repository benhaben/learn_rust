"""绿皮主题：条件怎么抽样。「至少有一个男孩」≠「先看到一个男孩」。

运行：python 04_boy_girl.py

两孩，各 50% 独立，样本空间 BB/BG/GB/GG 各 1/4。
信息 A：至少有一个男孩 → 去掉 GG，剩下三格，另一个也是男孩 = 1/3。
信息 B：随机敲门见到一个男孩 → 更接近 1/2（见到的那个已固定）。

口条：先写清楚「你怎么知道这个信息的」，再数格子。绿皮爱考这个坑。
"""

import numpy as np

rng = np.random.default_rng(4)
n = 50_000
# 0=女 1=男。两列两个孩子。
kids = rng.integers(0, 2, size=(n, 2))
n_boys = kids.sum(axis=1)

# A：至少一男。另一人也是男 = 两个都是男。
at_least = n_boys >= 1
p_both_given_al = (n_boys[at_least] == 2).mean()

# B：随机指出一个孩子，发现是男。再看另一个。
pick = rng.integers(0, 2, n)
seen = kids[np.arange(n), pick]
other = kids[np.arange(n), 1 - pick]
p_other_boy = other[seen == 1].mean()

print(f"至少一男 | 两男 {p_both_given_al:.3f}（应≈1/3）")
print(f"随机见到男 | 另一个也男 {p_other_boy:.3f}（应≈1/2）")
print("口条：条件是「至少」还是「指定位置」，样本空间不一样。")
