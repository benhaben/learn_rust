// 进阶：绑核。热线程钉在一颗核上，避免被调度器挪走丢掉 cache。
//
// 编译：g++ -O2 -std=c++17 11_affinity.cpp -o /tmp/11 && /tmp/11
//
// 生产还要：isolcpus / nohz_full、irq 赶到别的核、关 SMT、
// 内存和线程同一 NUMA。WSL 里能调用，看不到隔离效果。
//
// 忙等而不绑核：线程在核间跳，比 epoll 还抖。
//
// 口条：行情线程、策略线程、日志线程三颗核。热的两颗隔离，日志随便。

#include <cstdio>
#include <pthread.h>
#include <sched.h>
#include <unistd.h>

int main() {
    cpu_set_t set;
    CPU_ZERO(&set);
    CPU_SET(0, &set);
    if (pthread_setaffinity_np(pthread_self(), sizeof(set), &set) != 0) {
        std::perror("pthread_setaffinity_np");
        return 1;
    }
    cpu_set_t got;
    CPU_ZERO(&got);
    pthread_getaffinity_np(pthread_self(), sizeof(got), &got);
    std::printf("nproc=%ld  pinned_to_0=%d  sched_cpu=%d\n",
                sysconf(_SC_NPROCESSORS_ONLN), CPU_ISSET(0, &got), sched_getcpu());
    std::printf("口条：绑核是手段，目标是 p99。先 isolcpus 再 spin，别在共享核上空转。\n");
    return 0;
}
