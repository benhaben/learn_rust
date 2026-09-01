// 必考：伪共享。两线程写相邻 atomic，共享 64B 行，会来回作废。
//
// 编译：g++ -O2 -std=c++17 02_cache_line.cpp -o /tmp/02 -pthread && /tmp/02
//
// 交易里：行情线程写 seq、策略线程写自己的心跳，若挤在同一行，
// 两边 p99 都会抖。对策：alignas(64) 或中间垫 64B。
//
// 口条：锁包会变的数据；还会争的是缓存行，不是只有 mutex。

#include <atomic>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <thread>

constexpr int kIters = 5'000'000;

struct Shared {
    std::atomic<std::uint64_t> a{0};
    std::atomic<std::uint64_t> b{0};
};

struct Padded {
    alignas(64) std::atomic<std::uint64_t> a{0};
    alignas(64) std::atomic<std::uint64_t> b{0};
};

template <class T>
long long bench(const char* name) {
    T s;
    auto t0 = std::chrono::steady_clock::now();
    std::thread t1([&] {
        for (int i = 0; i < kIters; ++i) s.a.fetch_add(1, std::memory_order_relaxed);
    });
    std::thread t2([&] {
        for (int i = 0; i < kIters; ++i) s.b.fetch_add(1, std::memory_order_relaxed);
    });
    t1.join();
    t2.join();
    auto t1s = std::chrono::steady_clock::now();
    const long long us =
        std::chrono::duration_cast<std::chrono::microseconds>(t1s - t0).count();
    std::printf("%-8s  %lld us   sizeof=%zu  a=%llu\n", name, us, sizeof(T),
                static_cast<unsigned long long>(s.a.load()));
    return us;
}

int main() {
    auto slow = bench<Shared>("shared");
    auto fast = bench<Padded>("padded");
    std::printf("口条：相邻 atomic 伪共享。padded/shared ≈ %.2fx。生产看 perf c2c。\n",
                slow / static_cast<double>(fast));
    return 0;
}
