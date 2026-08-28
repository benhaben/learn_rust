"""必考：NumPy 切片常是 view；pandas 加法按 index 对齐。

运行：python 03_numpy_view.py
"""

import numpy as np
import pandas as pd

a = np.arange(4)
b = a[:2]
b[0] = 99
print("改 b 之后 a =", a, "  # a[0] 一起变了")

s1 = pd.Series([1.0, 2.0], index=["a", "b"])
s2 = pd.Series([10.0, 20.0], index=["b", "c"])
print("按标签相加\n", s1 + s2)
print("按位置相加", s1.to_numpy() + s2.to_numpy())
