// 必考：发布行情。先写 payload，再 Release flag；对面 Acquire 后再读。
//
// 编译：g++ -O2 -std=c++17 04_memory_order.cpp -o /tmp/04 -pthread && /tmp/04
//
// 和 rust_interview 30_atomic_order 同一张表：
//   Relaxed  只有原子性          计数器
//   Release  写：发布之前的写入
//   Acquire  读：之后能看见发布侧
//   SeqCst   全局总序            几乎不当默认
//
// 陷阱：flag 用 Relaxed、价用普通写，对面可能看到 ready=true、价还是 0。
// x86 上常「看起来能过」，面试和 ARM 会拆。
//
// 口条：先写数据，再 Release；Acquire 看到真之后再读数据。

#include <atomic>
#include <cstdint>
#include <cstdio>
#include <thread>

struct Tick {
    std::uint64_t px{0};
    std::uint64_t qty{0};
};

Tick g_tick;
std::atomic<bool> g_ready{false};

void publish(std::uint64_t px, std::uint64_t qty) {
    g_tick.px = px;
    g_tick.qty = qty;
    g_ready.store(true, std::memory_order_release);
}

bool consume(Tick& out) {
    if (!g_ready.load(std::memory_order_acquire)) return false;
    out = g_tick;
    return true;
}

int main() {
    std::thread pub([] { publish(101, 7); });
    pub.join();
    Tick t{};
    if (!consume(t)) {
        std::printf("join 之后仍未 ready，实现错了\n");
        return 1;
    }
    std::printf("Acquire 后 px=%llu qty=%llu\n",
                static_cast<unsigned long long>(t.px),
                static_cast<unsigned long long>(t.qty));
    std::printf("口条：flag Relaxed + 普通写 payload = 数据竞争。队列见 03。\n");
    return t.px == 101 && t.qty == 7 ? 0 : 1;
}
