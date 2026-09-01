# Python 量化面试示例

和 `rust_interview/` 里的 Rust bin 对应：每个主题一个可跑脚本。题面、口条见
[../rust_interview/python-quant-interview.md](../rust_interview/python-quant-interview.md)
和画布 `python-quant-interview.canvas.tsx`。

```bash
cd python_interview
python3 -m venv .venv
source .venv/bin/activate   # Windows: .venv\Scripts\activate
pip install -r requirements.txt

python 01_gil_numpy.py
python 04_returns.py
# …
```

| 编号 | 级别 | 文件 |
|---|---|---|
| 01–07 | 必考 | GIL、pandas、NumPy、收益、Sharpe、相关回归、泄漏 |
| 08–13 | 进阶/压轴 | walk-forward、特征标签、成本、仓位、IC、成交滞后 |
| 14–21 | AFML / Jansen 必要 | 三重屏障、dollar bar、分数差分、DSR、meta、分层、协整、MDA |
| 22 | 串起来 | purge/embargo 洗屏障事件 + 准确率 vs 扣费后净收益 |
| `dl/` | 深度学习（单独目录） | 序列泄漏、为何不先上 LSTM。JD 未写 DL 可后看 |

量化工程（C++ / Linux / FPGA）在 [`../sys_interview/`](../sys_interview/README.md)，和本目录互补：这里管研究正确性，那边管热路径延迟。
