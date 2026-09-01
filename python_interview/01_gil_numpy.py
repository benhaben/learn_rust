"""必考：GIL —— 为什么回测加速靠 NumPy / 多进程，不靠多线程

运行：python 01_gil_numpy.py

# 人话
GIL = CPython 的一把全局锁：同一时刻只允许一个线程跑 Python 字节码。
多线程在纯计算上加不上核；NumPy 把整段数组丢进 C 一次算完，才快。

# 目的
分清「等 I/O」和「算因子」该用什么：别以为开了 ThreadPool 回测就会变快。

# 场景
行情回调、等网络 → threading（线程会放开 GIL）。
百万根 K 线算收益/因子 → NumPy 向量化，或热路径用 Rust/C。
多组互相独立的回测 → multiprocessing（多进程，各一把 GIL）。

# 要说明什么

CPython 有一把 GIL（Global Interpreter Lock）：同一时刻只有一个线程在跑
Python 字节码。所以：

- 等网络 / 睡一会儿：线程会放开 GIL，threading 有用（行情回调）。
- 纯 Python for 循环算几百万根 K 线：几个线程也还是轮流用一颗核，
  回测不会因为 ThreadPool 就变快。

NumPy 的 np.log / np.diff 在 C 里扫整段数组，一次算完，不在 Python
里一根根循环。这才是「研究脚本变快」的常规办法。真要多核跑互相独立的
回测，用 multiprocessing，或把热路径写成 Rust/C 扩展。

# 本文件在做什么

四根价格 → 对数收益。等价于下面的 Python 循环，但循环在 C 里：

    logp[i] = log(px[i])
    ret[i]  = logp[i+1] - logp[i]

没有启动线程，所以你看不到 GIL 被卡住；看见的是「这种活应该向量化」。

# 向量化为啥能「一次进 C」

np.log 是 C 扩展。解释器把 ndarray 递进去一次，C 拿到 double* 和长度，
自己 for 扫完，再交回新数组。贵的是「进出解释器」；向量化整段只走一两趟
（log 一趟、diff 一趟），Python for 里每次 np.log(标量) 都是一趟。

px 本身仍是 Python 的 ndarray：形状、dtype、指针。数据区默认 float64，
和 C 的 double[] 一样紧挨着排。普通 list 没有这块缓冲，只是一排对象指针，
C 不能当 double[] 扫。array.array('d') 连续但几乎没有整段 log/diff。
""")

import numpy as np

# 铺成 C 认识的布局：对象头在 Python 侧，data 指向连续 double[4]
px = np.array([1.0, 1.01, 0.99, 1.02])

# 纯 Python 对照（慢在百万根上，四根看不出差别）
#
# 价格 4 根，收益只有 3 笔：从 i 涨到 i+1 才有一笔。所以 range(len(px)-1)。
#   i=0: log(1.01)-log(1.00)
#   i=1: log(0.99)-log(1.01)
#   i=2: log(1.02)-log(0.99)
#
# 列表推导式 [ 式子 for i in ... ] = 循环里 append 式子。外面的 [] 是造列表，
# 不是下标。for 管圈数，i 定下来之后才算式子。等价于：
#   ret_py = []
#   for i in range(len(px) - 1):
#       ret_py.append(np.log(px[i + 1]) - np.log(px[i]))
#
# 优先级：不是「右边先算再被 - 切开」。从里往外：
#   括号里 i+1 → px[…] 下标 → np.log(…) 调用 → 两边都得到数 → 最后相减。
# 减法只有一个 -，就是左 log 减 右 log。CPython 先算左边那个 log，再算右边。
# ** 才是从右结合；a-b-c 才是左结合 (a-b)-c。
ret_py = [np.log(px[i + 1]) - np.log(px[i]) for i in range(len(px) - 1)]

# 向量化：两次进 C（log 整段、diff 整段），循环在 C 里对着连续 double 扫。
# 快在少进解释器 + 缓存/SIMD，不是公式不同。只有四根时看不出快。
ret = np.diff(np.log(px))

print("价格", px)
print("Python 循环", ret_py)
print("NumPy      ", ret)
print("口条：等 I/O 用线程；算因子用 NumPy 或进程。threading 加不上回测算力。")
