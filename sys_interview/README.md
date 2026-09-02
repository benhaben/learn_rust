# 量化工程：C++ / Linux / FPGA

华钧这类岗位的主卷。研究侧口条仍看 [`../rust_interview/python-quant-interview.md`](../rust_interview/python-quant-interview.md)。

总表：[`quant-sys-interview.md`](quant-sys-interview.md)  
高频四块（热路径只是其中一块）见 [`../hft_interview/`](../hft_interview/README.md)。  
FPGA 专页：[`fpga.md`](fpga.md)  
画布（可筛选复习表）：[`quant-sys-interview.canvas.tsx`](/home/yin/.cursor/projects/home-yin-trading-learn-rust/canvases/quant-sys-interview.canvas.tsx)

```bash
cd sys_interview
chmod +x run.sh
./run.sh            # 编并跑全部 C++
./run.sh 03         # 只跑 03_spsc_ring
python3 10_latency_budget.py
```

| 编号 | 级别 | 文件 |
|---|---|---|
| 01 | 必考 | `01_hotpath.cpp` 热路径禁令、打包行情结构 |
| 02 | 必考 | `02_cache_line.cpp` 伪共享、`alignas(64)` |
| 03 | 必考 | `03_spsc_ring.cpp` 无锁 SPSC，行情→策略 |
| 04 | 必考 | `04_memory_order.cpp` Release/Acquire 发布 |
| 10 | 必考 | `10_latency_budget.py` 各层延迟、何时上 FPGA |
| 11 | 进阶 | `11_affinity.cpp` 绑核 |
| 12 | 进阶 | `12_clock.cpp` TSC vs `clock_gettime` |

C++ 要求：`g++ -O2 -std=c++17`，无第三方库。`11` 调 `sched_*`，只在 Linux 有意义（WSL 也能跑，效果弱于隔离核的生产机）。
