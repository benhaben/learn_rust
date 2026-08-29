# Python 量化面试题集

可跑代码在仓库根目录 [`python_interview/`](../python_interview/README.md)（`01_*.py` … `13_*.py`）。

可筛选复习表（和 Rust 表同一套交互）：打开 Cursor 画布
`python-quant-interview.canvas.tsx`。

研究用 Python，热路径用 Rust / Nautilus。两边必须共用「信号时刻 vs 成交时刻」。

## 必考

| 主题 | 一句话 | 典型问法 |
|---|---|---|
| GIL / 线程 / 进程 | CPU 循环加不上核；I/O 可以；矩阵交给 NumPy | 回视为何不用 threading 翻倍 |
| pandas 链式赋值 | 用 `.loc[mask, col]=`，别链式切 | 为什么有时赋值没写进去 |
| loc vs iloc | loc 按标签，iloc 按位置；默认 0,1,2 碰巧重合 | 删一行后 loc[2] 和 iloc[2] 谁还在 |
| NumPy view / 对齐 | 切片常共享内存；Series 按 index 加 | 两个等长 Series 为何对不齐 |
| 简单 vs 对数收益 | 组合用简单；时间可加用对数 | 净值为何 `(1+R).cumprod` |
| 波动年化 | `σ_日 × √252`，写清交易日 | 1% 日波动年化多少、为何根号 |
| Sharpe / MDD | 超额/波动；峰到谷；要扣费 | Sharpe 一样为何还看回撤换手 |
| 相关 / 协方差 | 对**收益**相关；危机相关趋向 1 | 为何别对价格做相关 |
| OLS / beta | 预测必须 `y_t ~ X_{t-1}` | 残差能当 alpha 吗 |
| 时间泄漏 | 禁止随机 `train_test_split` | 为何 AUC 虚高 |
| 成本滑点 | 净 = 毛 − \|Δ仓\|×(费+滑点) | 年换手 100%、10bp 吃多少 |

## 进阶

| 主题 | 一句话 |
|---|---|
| Walk-forward / purged CV | 标签重叠要 purge，结束后 embargo |
| 标签 | 下期收益 vs 三重屏障（止盈/止损/超时） |
| 过拟合 | 尝试次数吹 Sharpe；持有集只碰一次 |
| 特征 | 平稳变换：收益、波动、失衡；不用原始价 |
| 平稳 / 协整 | 配对看协整不是只看相关 |
| 多重检验 | 扫 100 因子必出星号；看 FDR 与经济显著 |
| IC / IR | 日截面 IC 很小也能用；IR≈IC×√有效广度 |
| 仓位 | 信号≠仓位；波动目标；分数 Kelly |
| 线性 vs 树 | 短样本、要中性化先线性；树要浅且时间外验证 |
| 不平衡 | 别看 accuracy；看扣费后分位数多空 |

## 压轴

| 主题 | 一句话 |
|---|---|
| 机制切换 | 机制特征必须因果，不能用未来涨跌定义牛市 |
| 信号到成交 | 信号 t，成交至少 t+1 开/VWAP |
| Point-in-time | 成分、财报、分析师都要 asof；防存活偏差 |
| 在线更新 | 每笔预测记 model_id + 特征哈希，否则不能复盘 |

## ML for algo trading 八问

1. 特征 ≤ t，标签在未来？  
2. 标准化只 fit 训练折？  
3. CV 是否 purge / embargo？  
4. 评估是否扣费后的钱，不是准确率？  
5. 持有集是否锁死？  
6. 特征是否平稳（不是价格水平）？  
7. 成交是否在信号之后？  
8. 线上预测能否按日复现？

## 公式卡

- \(R = P_1/P_0 - 1\)，\(r = \log(1+R)\)  
- \(\sigma_{\mathrm{year}} \approx \sigma_{\mathrm{day}}\sqrt{252}\)  
- Sharpe \(= \mathrm{E}[R-r_f]/\sigma \times \sqrt{252}\)  
- MDD \(= \min(\mathrm{NAV}/\mathrm{cummax}(\mathrm{NAV})-1)\)  
- 年成本粗算：换手 × 单边成本 × 2  
- Kelly \(f^* = \mu/\sigma^2\)（实盘用分数）

## 技术栈

pandas/polars、numpy、statsmodels、sklearn/lightgbm、回测（自研或 Nautilus）、执行层与研究共用滞后规则。
