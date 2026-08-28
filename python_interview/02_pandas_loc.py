"""必考：用 .loc 一次赋值，不要 df[df.a>0]['b']=1。

运行：python 02_pandas_loc.py
"""

import pandas as pd

df = pd.DataFrame({"ret": [0.01, -0.02, 0.03], "side": [0, 0, 0]})
df.loc[df["ret"] > 0, "side"] = 1
print(df)
# 不要: df[df.ret > 0]["side"] = 1  # 可能改在副本上
