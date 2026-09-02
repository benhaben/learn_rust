# Python 量化面试示例

和 `rust_interview/` 里的 Rust bin 对应：每个主题一个可跑脚本。题面、口条见
[../rust_interview/python-quant-interview.md](../rust_interview/python-quant-interview.md)
和画布 `python-quant-interview.canvas.tsx`。

每个 `.py` 顶部文档字符串都有三块：**人话**（这是什么）、**目的**、**场景**（什么时候用）。
先读这三块再跑代码。

**差分就是相减。** 一阶差分 = 今天减昨天（`P_t − P_{t−1}`，对数上就是收益）。
分数差分（16）= 今天减去「很多天以前的加权和」，越远权重越小，为的是更平稳又少丢价格水平。
`np.convolve` 是一维滑窗加权（不是 CNN、不是一般矩阵乘法）；均线是特例。
生 close 不要当特征；用通道位置、离高点%、z-score、截面 rank 这类相对化水平。

```bash
cd python_interview
python3 -m venv .venv
source .venv/bin/activate   # Windows: .venv\Scripts\activate
pip install -r requirements.txt

python 01_gil_numpy.py
python 04_returns.py
# …
```

| 编号 | 级别 | 文件 | 这一份在证明什么 |
|---|---|---|---|
| 01 | 必考 | `01_gil_numpy.py` | CPython 有 GIL：回测算力靠 NumPy / 多进程，不靠多线程 |
| 02 | 必考 | `02_pandas_loc.py` | 过滤赋值用 `.loc`；链式切片赋值可能改在副本上 |
| 03 | 必考 | `03_numpy_view.py` | NumPy 切片常共用内存；pandas 加减按标签对齐 |
| 04 | 必考 | `04_returns.py` | 组合用简单收益；对数收益多期可加 |
| 05 | 必考 | `05_vol_sharpe.py` | 波动年化乘 √252；Sharpe / Sortino / 最大回撤 |
| 06 | 必考 | `06_corr_ols.py` | 相关对收益不对价格；预测回归必须滞后因子 |
| 07 | 必考 | `07_leakage_split.py` | 按时间切分；标准化只 fit 训练集 |
| 08 | 进阶 | `08_walk_forward.py` | walk-forward；purge 洗重叠标签，embargo 封测试后一段 |
| 09 | 进阶 | `09_features_label.py` | 特征用收益/波动，不要收盘价；标签=下一根收益 |
| 10 | 必考 | `10_cost.py` | 昨仓 × 今收益；净收益还要扣换手成本 |
| 11 | 进阶 | `11_position.py` | 信号÷波动再归一，再缩放到目标波动才是仓位 |
| 12 | 进阶 | `12_ic.py` | 截面 IC：当天因子 vs 下期收益；Spearman 抗幅度、不抗两头名次 |
| 13 | 压轴 | `13_execution_lag.py` | 信号用 t，收益至少用下一根；shift(1) 不是信号失效 |
| 14 | AFML | `14_triple_barrier.py` | 标签=先碰到止盈 / 止损 / 超时；路径只打 y 不进 X |
| 15 | AFML | `15_dollar_bar.py` | 成交额凑满才收一根，时钟 bar 会扭曲信息密度 |
| 16 | AFML | `16_frac_diff.py` | 分数差分：比一阶差分少丢价格记忆，又比原价更平稳 |
| 17 | AFML | `17_deflated_sharpe.py` | 参数试得越多，最好的 Sharpe 越容易被运气抬高，要打折 |
| 18 | AFML | `18_meta_label.py` | 元标签=主模型对不对；主定方向，次定下不下 |
| 19 | AFML | `19_quantile_ls.py` | 分层多空：Pearson/Spearman/组均值各挡一种假；Q5−Q1 是多空价差 |
| 20 | AFML | `20_cointegration.py` | 配对看价差是否平稳，价格相关高不等于能配对 |
| 21 | AFML | `21_mda_importance.py` | 重要性在时间外置换（MDA）；别信训练里的 MDI |
| 22 | 串起来 | `22_purge_net.py` | 把 14 的标签接到 08 的切分和 10 的扣费；看净不看 accuracy |
| `dl/` | 后看 | `dl/01_seq_leakage.py`、`dl/02_why_not_first.py` | LSTM 不免疫泄漏；短样本不要先上深度学习 |

量化工程（C++ / Linux / FPGA）在 [`../sys_interview/`](../sys_interview/README.md)，和本目录互补：这里管研究正确性，那边管热路径延迟。

经典量化面试数学（绿皮书主题，不抄原文）在 [`../greenbook_interview/`](../greenbook_interview/README.md)。

高频时序（微观 / Hawkes / HJB / 热路径）在 [`../hft_interview/`](../hft_interview/README.md)。
