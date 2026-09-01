// 必考：热路径禁令 + 打包行情
//
// 编译：g++ -O2 -std=c++17 01_hotpath.cpp -o /tmp/01 && /tmp/01
//
// 行情回调里一笔 tick 绝不要：
//   heap 分配 / 锁 / 日志 / 异常 / 虚调用 / syscall
// 研究脚本可以；热路径不行。中频股票的 on_bar 也尽量预分配。
//
// 交易所二进制是按字节排的。编译器默认对齐会在字段间塞垫片，
// sizeof 对不上、memcpy 会读错。对包要用 pack，对热数据用 int 符号 id。

#include <cstdint>
#include <cstdio>
#include <cstddef>
#include <string>
#include <vector>

#if defined(__GNUC__)
#define PACKED __attribute__((packed))
#else
#define PACKED
#endif

struct QuoteHot {
    std::int32_t inst;
    std::int32_t bid_px;  // 价用整型 tick，避免热路径 double 舍入扯皮
    std::int32_t ask_px;
    std::int32_t bid_sz;
    std::int32_t ask_sz;
    std::uint64_t exch_ts;
};

#if defined(__GNUC__)
#pragma pack(push, 1)
#endif
struct QuoteWire {
    std::int32_t inst;
    std::int32_t bid_px;
    std::int32_t ask_px;
    std::int32_t bid_sz;
    std::int32_t ask_sz;
    std::uint64_t exch_ts;
} PACKED;
#if defined(__GNUC__)
#pragma pack(pop)
#endif

struct QuoteResearch {
    std::string symbol;
    std::vector<double> depth;
    double bid_px;
};

int main() {
    std::printf("QuoteHot      %zu  热路径：固定布局、无堆\n", sizeof(QuoteHot));
    std::printf("QuoteWire     %zu  电文：pack 后应 = 4*5+8 = 28\n", sizeof(QuoteWire));
    std::printf("QuoteResearch %zu  研究：string/vector 各一个堆指针\n",
                sizeof(QuoteResearch));
    std::printf("offset ask_px hot=%zu wire=%zu\n",
                offsetof(QuoteHot, ask_px), offsetof(QuoteWire, ask_px));
    std::printf("口条：tick 里用 POD + 预分配。string 做符号表，热路径只传 inst id。\n");
    return 0;
}
