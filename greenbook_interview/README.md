# 量化面试「绿皮书」主题（可跑脚本）

对应的是 Xinfeng Zhou《A Practical Guide To Quantitative Finance Interviews》（绿皮封面）。  
书里的题面和解答有版权，**这里不抄原文**，按同一知识块写成可复现的小实验：概率、随机过程、期权。

本仓库另外三块不重复：

| 目录 | 管什么 |
|---|---|
| [`../python_interview/`](../python_interview/README.md) | 研究正确性：泄漏、成本、屏障、dollar bar |
| [`../sys_interview/`](../sys_interview/README.md) | C++ / Linux / FPGA 执行层 |
| [`../rust_interview/`](../rust_interview/README.md) | Rust 语言 |

```bash
cd greenbook_interview
python3 01_monty_hall.py
python3 13_bs_mc.py
```

只需标准库 + NumPy（和 `python_interview` 同一环境即可）。

| 编号 | 绿皮书主题块 | 文件 | 口条 |
|---|---|---|---|
| 01 | 信息会改概率 | `01_monty_hall.py` | 开门之后不是 1/2 |
| 02 | 组合 / 碰撞 | `02_birthday.py` | 23 人就过 50% |
| 03 | 贝叶斯 | `03_bayes_test.py` | 患病率低，阳也不一定有病 |
| 04 | 条件样本空间 | `04_boy_girl.py` | 「至少一个」怎么抽样 |
| 05 | 条件期望 | `05_cond_expect.py` | 塔性质：先条件再无条件 |
| 06 | 无记忆 | `06_memoryless.py` | 指数 / 几何，已等过的不算 |
| 07 | 泰勒 | `07_taylor.py` | 本地多项式，不是全局 |
| 08 | 线性代数 / 风险 | `08_eigen_cov.py` | 协方差特征方向 = 主风险 |
| 09 | 随机游走 | `09_gambler_ruin.py` | 有限资本会被吸干 |
| 10 | 布朗运动 | `10_brownian.py` | 均值 0，方差 = t |
| 11 | Itô | `11_ito_dw2.py` | (dW)² 像 dt，不是 0 |
| 12 | 期权平价 | `12_put_call.py` | C−P = 远期 |
| 13 | Black–Scholes | `13_bs_mc.py` | 闭式 ≈ 风险中性 MC |
| 14 | Greeks | `14_delta_bump.py` | Δ 是 bump，也是 n(d1) |
| 15 | 对数正态 | `15_lognormal.py` | 均值被右尾拉高，中位数更小 |

建议顺序：01–06 概率（面得最多）→ 09–11 随机 → 12–15 衍生品。07–08 当补丁。
