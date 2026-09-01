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
| 08–12 | 进阶 | walk-forward、标签屏障、特征、成本、仓位 |
| 13 | 压轴 | 信号到成交的滞后 |
