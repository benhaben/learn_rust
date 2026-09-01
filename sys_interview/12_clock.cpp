// 进阶：测延迟用哪只钟。墙上时钟会 NTP 跳；热路径间隔用 TSC 或 RAW。
//
// 编译：g++ -O2 -std=c++17 12_clock.cpp -o /tmp/12 && /tmp/12
//
// CLOCK_REALTIME        墙上时间，会跳，只打日志
// CLOCK_MONOTONIC       不往回跳，仍可能被 NTP 微调
// CLOCK_MONOTONIC_RAW   不调频，测间隔更稳
// rdtsc                 一记指令；跨核前确认 constant_tsc
//
// FPGA / 交易所时间是另一套钟。用主机 now() 减板上时间会撒谎。
//
// 口条：报 p99 先说钟。校准 TSC，跨核、跨机用交易所时间戳或 PTP。

#include <cstdint>
#include <cstdio>
#include <ctime>
#include <x86intrin.h>

static long long ns_of(clockid_t id) {
    timespec ts{};
    clock_gettime(id, &ts);
    return static_cast<long long>(ts.tv_sec) * 1000000000LL + ts.tv_nsec;
}

int main() {
    const auto a = ns_of(CLOCK_MONOTONIC_RAW);
    const auto t0 = __rdtsc();
    for (volatile int i = 0; i < 100000; ++i) {
    }
    const auto t1 = __rdtsc();
    const auto b = ns_of(CLOCK_MONOTONIC_RAW);
    std::printf("RAW delta     %lld ns\n", b - a);
    std::printf("rdtsc delta   %llu  (cycles，不是 ns，要除频率)\n",
                static_cast<unsigned long long>(t1 - t0));
    std::printf("口条：间隔用 RAW/TSC；对账用交易所 ts。别把三种钟减出来当延迟。\n");
    return 0;
}
