"""必考：延迟账。先画 hop，再决定旁路还是 FPGA。

运行：python3 10_latency_budget.py

数字是口条数量级，不是某家柜台的 SLA。面试用它证明你会拆，
不会背「亚微秒」当事实。

    内核 TCP 收包     10–50 μs     中断 + 拷贝 + 协议
    内核旁路          1–5  μs     去掉内核，策略仍在 CPU
    FPGA 解析+DMA     0.2–1 μs    决策若回主机，PCIe 已占一截
    FPGA tick-to-trade 0.2–1 μs   触发必须在板上
    系统调用 / 日志   0.2–2 μs    热路径禁止
    跨 NUMA           ~0.1 μs     绑核就要绑内存

中频股票（秒到分钟）：边在 PIT、成本、成交滞后，不在 2μs。
期货抢价：每一跳都要能指着这张表说。
"""

HOPS = [
    ("光纤/交换", 0.05, "共址后常不是最大头"),
    ("内核 TCP", 20.0, "含中断、拷贝；抖动大"),
    ("旁路 NIC", 2.0, "Onload/DPDK，策略仍是 C++"),
    ("软件解析+簿记", 1.5, "分支、cache；可下 FPGA"),
    ("策略 CPU", 1.0, "中频可更慢；高频必须绑核忙等"),
    ("软件风控+组包", 0.8, "硬门槛可下板"),
    ("内核发送", 15.0, "旁路可压到 1–3"),
]

FPGA_ON = {
    "内核 TCP": 0.0,
    "旁路 NIC": 0.3,  # MAC/PCS 仍在
    "软件解析+簿记": 0.2,
    "策略 CPU": 0.0,  # tick-to-trade 假定触发在板
    "软件风控+组包": 0.2,
    "内核发送": 0.0,
}


def path(name, hops):
    total = sum(us for _, us, _ in hops)
    print(f"\n{name}  合计 ~{total:.1f} μs")
    for hop, us, note in hops:
        bar = "#" * max(1, int(us))
        print(f"  {hop:<14} {us:6.1f} μs  {bar}  {note}")
    return total


def main():
    soft = [(h, u, n) for h, u, n in HOPS if h != "旁路 NIC"]
    bypass = []
    for h, u, n in HOPS:
        if h == "内核 TCP":
            continue
        if h == "内核发送":
            bypass.append((h, 2.0, "旁路发送"))
        else:
            bypass.append((h, u, n))
    fpga = []
    for h, u, n in HOPS:
        if h in FPGA_ON:
            fpga.append((h, FPGA_ON[h], "下板或消失" if FPGA_ON[h] == 0 else n))
        else:
            fpga.append((h, u, n))

    a = path("A  内核协议栈（研究机/未调优）", soft)
    b = path("B  内核旁路 + CPU 策略（常见期货热路径）", bypass)
    c = path("C  FPGA tick-to-trade（触发在板）", fpga)

    print("\n口条：")
    print(f"  旁路相对内核大约 {a / b:.0f}x，FPGA 再去掉解析/内核（本表 ~{b / c:.0f}x）。")
    print("  中频股票走 A 也够；先问瓶颈是 hop 还是研究/冲击。")
    print("  FPGA 买确定延迟，不是更聪明的模型。没交过板就说切分，别编 RTL。")


if __name__ == "__main__":
    main()
