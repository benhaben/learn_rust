// 必考：单产单消无锁环。行情线程 → 策略线程的默认管道。
//
// 编译：g++ -O2 -std=c++17 03_spsc_ring.cpp -o /tmp/03 -pthread && /tmp/03
//
// 为什么不是 mutex + std::queue：
//   锁会睡、会抖、会进内核；多生产者才需要 MPSC/MPMC。
// 单行情源 + 单策略热线程：SPSC 足够，也最好证。
//
// 规则：容量 2^n；生产者只动 tail，消费者只动 head；
// 对面下标用 Acquire 读。槽本身先写完再发布下标（见 04）。
//
// 口条：热路径队列只有一个写手、一个读手。多策略就多条环，别把 mutex 塞进 tick。

#include <atomic>
#include <cstdint>
#include <cstdio>
#include <thread>
#include <vector>

template <class T, std::size_t N>
struct Spsc {
    static_assert((N & (N - 1)) == 0, "N 必须是 2 的幂");
    T slot[N];
    alignas(64) std::atomic<std::size_t> tail{0};  // 下一格要写
    alignas(64) std::atomic<std::size_t> head{0};  // 下一格要读

    bool push(const T& v) {
        const auto t = tail.load(std::memory_order_relaxed);
        const auto h = head.load(std::memory_order_acquire);
        if (t - h == N) return false;
        slot[t & (N - 1)] = v;
        tail.store(t + 1, std::memory_order_release);
        return true;
    }

    bool pop(T& out) {
        const auto h = head.load(std::memory_order_relaxed);
        const auto t = tail.load(std::memory_order_acquire);
        if (h == t) return false;
        out = slot[h & (N - 1)];
        head.store(h + 1, std::memory_order_release);
        return true;
    }
};

int main() {
    Spsc<int, 1024> q;
    constexpr int kN = 100000;
    std::thread prod([&] {
        for (int i = 0; i < kN;) {
            if (q.push(i)) ++i;
        }
    });
    long long sum = 0;
    int got = 0, x = 0;
    while (got < kN) {
        if (q.pop(x)) {
            sum += x;
            ++got;
        }
    }
    prod.join();
    const long long expect = static_cast<long long>(kN - 1) * kN / 2;
    std::printf("got=%d sum=%lld expect=%lld\n", got, sum, expect);
    std::printf("口条：SPSC = 预分配环 + 头尾分缓存行 + Acquire/Release。满了丢还是背压，策略先定。\n");
    return sum == expect ? 0 : 1;
}
