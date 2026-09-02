# 高频时序面试题

对应口述四块：微观结构 → Hawkes → HJB → C++ 热路径。  
研究正确性仍看 [`../python_interview/`](../python_interview/README.md)；工程细节看 [`../sys_interview/`](../sys_interview/README.md)。  
题面口条总表：[`hft-timeseries-interview.md`](hft-timeseries-interview.md)。

```bash
cd hft_interview
python3 01_p99_vs_mean.py
python3 02_ofi.py
python3 03_hawkes.py
python3 04_kyle.py
python3 05_inventory_spread.py
```

只需 NumPy（与 `python_interview` 同一环境）。

| 编号 | 块 | 文件 | 口条 |
|---|---|---|---|
| 01 | C++ | `01_p99_vs_mean.py` | 均值好看，p99 才是被抢先的时刻 |
| 02 | 微观 | `02_ofi.py` | OFI = 谁在主动；不是你这单为啥成交 |
| 03 | Hawkes | `03_hawkes.py` | 底噪 + 余波；泊松不会成簇 |
| 04 | 微观 | `04_kyle.py` | 你买多少，价格推多少 |
| 05 | HJB | `05_inventory_spread.py` | 货多往下挪；怕风险、波动大则价差宽 |

阶段 A 从 OFI / 冲击 / 队列开始，不要从 HJB 方程开始。
