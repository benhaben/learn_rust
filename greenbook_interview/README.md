# 量化面试「绿皮书」主题（可跑脚本）

对应的是 Xinfeng Zhou《A Practical Guide To Quantitative Finance Interviews》（绿皮封面）。  
书里的题面和解答有版权，**这里不抄原文**，按同一知识块写成可复现的小实验。

本仓库另外三块不重复：

| 目录 | 管什么 |
|---|---|
| [`../python_interview/`](../python_interview/README.md) | 研究正确性：泄漏、成本、屏障、dollar bar |
| [`../sys_interview/`](../sys_interview/README.md) | C++ / Linux / FPGA 执行层 |
| [`../rust_interview/`](../rust_interview/README.md) | Rust 语言 |

```bash
cd greenbook_interview
python3 01_monty_hall.py
python3 26_bs_pde.py
```

只需标准库 + NumPy（和 `python_interview` 同一环境即可）。

## 前半：概率 · 计算 · 随机入门 · BS

| 编号 | 主题 | 文件 | 口条 |
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
| 14 | Delta | `14_delta_bump.py` | Δ 是 bump，也是 Φ(d1) |
| 15 | 对数正态 | `15_lognormal.py` | 均值被右尾拉高，中位数更小 |

## 后半：鞅 · 反射 · 利率 · 美式 · PDE · 算法

面试后半最爱追问的，前 15 个没覆盖到的缺口：

| 编号 | 主题 | 文件 | 口条 |
|---|---|---|---|
| 16 | Jensen | `16_jensen.py` | 凸函数不能和期望换序；波动值钱 |
| 17 | 泊松过程 | `17_poisson.py` | 计数 ~Pois(λt)，等待 ~Exp(λ) |
| 18 | 次序统计 | `18_order_stat.py` | n 个 U(0,1)，E[min]=1/(n+1) |
| 19 | 鞅 / 可选停 | `19_martingale.py` | E[下一刻\|现在]=现在 |
| 20 | BM 最大值 | `20_bm_max.py` | 反射：P(碰到 a)≈2P(终点≥a) |
| 21 | Cholesky | `21_chol_corr.py` | 独立正态拧成相关 |
| 22 | P vs Q | `22_p_vs_q.py` | 统计用 μ，定价用 r |
| 23 | 复利 / 远期 | `23_compound.py` | 先问约定；连续是 e^{rt} |
| 24 | 久期 / DV01 | `24_duration.py` | 一阶敏感；1bp 是多少钱 |
| 25 | 美式看涨 | `25_american_call.py` | 无股息不提前行权 |
| 26 | BS PDE | `26_bs_pde.py` | Θ + rSΔ + ½σ²S²Γ = rC |
| 27 | 远期 vs 期货 | `27_fwd_fut.py` | r 确定则相等 |
| 28 | Box–Muller | `28_box_muller.py` | 两个均匀 → 一对独立正态 |

建议：前半 01–06 再 12–15；后半先 **16、19、22、25、26**（口条最常被要），再补 17–21、23–24、27–28。

书后还有大量脑筋急转弯和手推证明，这里不做成脚本。口条够用即可，具体数字以你手推为准。
