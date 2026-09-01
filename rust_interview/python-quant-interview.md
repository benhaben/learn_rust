# Python 量化面试题集

可跑代码在仓库根目录 [`python_interview/`](../python_interview/README.md)（`01_*.py` … `21_*.py`）。
深度学习（Jansen 后半本）在 [`python_interview/dl/`](../python_interview/dl/README.md)，不列入面试主线。

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
| 简单 vs 对数收益 | 组合用简单；时间可加用对数。`log`/`exp` 都是 e 为底；`exp(Σr)=Pn/P0` | 净值为何 `(1+R).cumprod`；`0.1906` 和 `1.21` 差在哪 |
| 波动年化 | `σ_日 × √252`，写清交易日 | 1% 日波动年化多少、为何根号 |
| Sharpe / MDD | 超额/波动；峰到谷；要扣费 | Sharpe 一样为何还看回撤换手 |
| 相关 / 协方差 | 对**收益**相关；危机相关趋向 1。`corrcoef` 返回矩阵，ρ 在 `[0,1]` | 为何别对价格做相关 |
| OLS / beta | 预测必须 `y_t ~ X_{t-1}`。`lstsq`=最小二乘；X 第一列全 1 是截距 | 残差能当 alpha 吗；`beta[0]`/`beta[1]` 是谁 |
| 时间泄漏 | 禁止随机切分。`shuffle`=洗牌，金融不能洗；标准化只 fit 训练折 | 为何 AUC 虚高；测试折均值不是 0 为何对 |
| 成本滑点 | 净 = 毛 − \|Δ仓\|×(费+滑点) | 年换手 100%、10bp 吃多少 |

## 进阶

| 主题 | 一句话 |
|---|---|
| Walk-forward / purged CV | purge=清洗重叠标签；embargo=测试后封禁一段 |
| 三重屏障 | 先碰到止盈 / 止损 / 超时；屏障随波动缩放 | 
| dollar bar | 成交额凑满收一根，不是均匀时钟 |
| 分数差分 | d∈(0,1) 要平稳又少丢记忆；d=1 就是普通差分 |
| DSR / 试次 | 扫 N 次，最大 Sharpe 期望被吹高；持有集只碰一次 |
| meta-labeling | 主模型定方向，次模型定下不下 |
| 分位数多空 | 分层收益要单调，只报 IC 不够 |
| 标签 | 下期收益 vs 三重屏障（止盈/止损/超时） |
| 过拟合 | 尝试次数吹 Sharpe；持有集只碰一次 |
| 特征 | 平稳变换：收益、波动、失衡；不用原始价 |
| 平稳 / 协整 | 配对看协整不是只看相关；价格相关可以虚高 |
| 多重检验 | 扫 100 因子必出星号；看 FDR 与经济显著 |
| IC / IR | 截面 IC=当天因子 vs 下期收益。`0.1×因子+噪声` 是造假数据不是因子公式 |
| 仓位 | 信号≠仓位；个股 vol 定比例，组合 vol 定杠杆，对上产品目标波动 |
| 线性 vs 树 | 短样本、要中性化先线性；树要浅且时间外验证 |
| MDA 重要性 | 时间外置换；别信训练里的 MDI |
| 不平衡 | 别看 accuracy；看扣费后分位数多空 |

## 压轴

| 主题 | 一句话 |
|---|---|
| 机制切换 | 机制特征必须因果，不能用未来涨跌定义牛市 |
| 信号到成交 | `shift(1)` 用昨信号赚今收益，不是信号过期；今信号×今收益是偷看 |
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
9. 标签是否三重屏障或明确地平线，屏障是否按波动缩放？  
10. 报 Sharpe 前是否说清试了多少次（DSR / 持有集）？

## 公式卡

- \(R = P_1/P_0 - 1\)，\(r = \ln(1+R)\)（`np.log` 是 e 为底；\(\mathrm{e}^{\sum r}=P_n/P_0\)）  
- \(\sigma_{\mathrm{year}} \approx \sigma_{\mathrm{day}}\sqrt{252}\)  
- Sharpe \(= \mathrm{E}[R-r_f]/\sigma \times \sqrt{252}\)  
- MDD \(= \min(\mathrm{NAV}/\mathrm{cummax}(\mathrm{NAV})-1)\)  
- 年成本粗算：换手 × 单边成本 × 2  
- Kelly \(f^* = \mu/\sigma^2\)（实盘用分数）

## 技术栈

pandas/polars、numpy、statsmodels、sklearn/lightgbm、回测（自研或 Nautilus）、执行层与研究共用滞后规则。
