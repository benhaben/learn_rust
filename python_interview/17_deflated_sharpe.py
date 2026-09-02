"""进阶：参数试得越多，最好的那个 Sharpe 越容易被运气抬高，不能当真。

运行：python 17_deflated_sharpe.py

# 人话

Sharpe 是「多赚的钱 / 抖得有多厉害」。2 听起来很猛。

但你不是只测了一条策略。网格搜参时可能试了 20 组、100 组，最后只拿出
最大的那个报：「我找到 Sharpe=2 了」。

问题：即使每条策略都是瞎猜（真 alpha=0，真 Sharpe 该在 0 附近晃），
试的次数一多，里面总会有几个运气好的。你专挑最大的，这个最大值
平均就会大于 0，试得越多，被吹得越高。

这不是程序写错了，是「只报冠军」自带的偏差，统计上叫多重检验。
所以搜参扫出来的 2，要当注了水的数，得打折。

# 目的

提醒：回测成绩好看，先问试了多少组。组数多，冠军 Sharpe 就要往下修。

AFML 里的 Deflated Sharpe（DSR，紧缩夏普）就是在做这件事：
    以前：和 0 比，大于 0 就算有本事
    现在：先估计「试 N 次、全是噪声时，最大 Sharpe 大概能到多少」
          再看你的样本 Sharpe 有没有明显高于这个门槛
门槛被抬高了，注水的 2 往往就不显著了。

本文件不推公式，用模拟代替：造 N 条纯噪声日收益，各算一年 Sharpe，
打印这 N 个里最大的那个。你会看到 N=1 还小，N=100 就明显变大。

# 「噪声」在这里指什么（和你调特征的关系）

本文件的噪声很具体：日收益 ~ N(0, 1%)，均值就是 0。
翻译成人话：这条「策略」对未来没有预测力，盈亏是运气。
真 Sharpe=0。一年只有 252 天，样本 Sharpe 仍会在 0 附近乱晃，
偶尔晃到 1、2，那是抽样误差，不是找到了 alpha。

研究里说「全是噪声」不是说行情是白噪声，而是这个意思：
    你试的那些特征 / 参数，和未来收益其实没关系。
    回测里看起来能赚钱，只是这段历史碰巧配合。

你在调测试特征和特征参数，每一次换窗口、换阈值、换 z 的算法，
都算一次试验，N 就是你试过的组合数（相关的组合要折成有效 N，
100 个长得很像的参数，有效次数远小于 100）。
最后只报最好的那一组 Sharpe，和本文件「只报最大值」是同一件事。

注意两件更糟的：
    1) 在测试集上调特征和参数 = 测试集已经当训练集用了，N 次全漏进去。
       测试集只能最后看一次，用来拍板，不能拿来搜。
    2) 本文件的门槛是「独立噪声、一年」的示意。真 DSR 还要考虑
       收益偏度、峰度、试验是否独立。口条先记住：先数 N，再打折。

# 固定策略不用「制造噪声」——那是对照，不是实盘

不要往真策略、真行情里加随机数。噪声只出现在脑子里这一问：
    假如这条规则其实没本事，我现在看到的 Sharpe 常见吗？

实盘仍跑你那条固定规则。检验在研究脚本里做，有三条能落地的路：

    1) 最重要：换一段没调过参的样本再跑同一条规则（持有集 / walk-forward，见 08）。
       规则冻结，数字掉下来，多半就是之前的运气。这不需要造噪声。

    2) 置换：价格、成交规则都不变，把标签或收益顺序打乱，再套同一条规则，
       重复很多遍。打乱后特征和未来不该再有关系，得到的 Sharpe 分布
       就是「没本事时」的样子。你的真实 Sharpe 要明显落在这堆的右边。

    3) 公式 / 本文件这种模拟：承认试过 N 组，用「真 Sharpe=0、长度 T」
       估冠军大概能被吹到多高（DSR）。不用改策略，只用来抬门槛。

已经冻结、而且选规则之前没在这段数据上搜过（N=1），多重检验不严重。
但一年样本 Sharpe 仍会晃，所以还是要靠样本外，不是靠往仓位里加点噪声。
若冻结前已经搜过 50 组，只是现在不改了，N 仍是 50，不是 1。

# 场景

最糟的讲法：网格搜参、因子海选，只把最好的一条回测拿去讲。
更好的看盘方式（你在用的）：
    平坦高原  参数左右挪一挪，成绩差不多，而不是只有一个针尖特别高。
              针尖多半是这段历史的运气；高原说明规则不靠某一个魔法数字。
    多周期    同一条规则在好几段时间都还行，不是只在某一两年好看。
              这是简陋的稳健性 / 非正式 walk-forward。

这两条能挡掉很多「冠军幻觉」，但仍替代不了持有集：
    高原和多周期如果都在你调参时看过的数据上完成，N 仍然变大了
    （你看了整张参数表、又切了好几段来挑「都有效」的）。
    拍板还要留一段从没用来选高原、也没用来比周期的样本，只跑一次。
持有集只能看一次，不能拿它再调参。这和 08 的 purge（洗重叠标签）不是同一件事。

# 实际怎么用：同一条真实行情，你做一次，随机买卖做 N 次

尺子也要吃真实数据。正确对照是：
    左边  真实行情 + 你的规则 + 昨仓×今收益 − 换手费（10）→ 你的 Sharpe
    右边  同一条行情 + 随机买卖（换手次数和你差不多）+ 同一套扣费
          重复 n_tried 次，取最大 Sharpe → 门槛

两边波动、涨跌日、成本口径一样，才比得过。
上半段那种「另造一列 N(0,1%)」只是面试示意，和你的票无关，实际不要当尺子。

随机买卖不要天天乱翻：换手会把净 Sharpe 砸到很负，门槛太好过，没有鉴别力。
右边应和你的策略换手相当（本文件按你的翻仓次数随机切段）。

# 本文件在做什么

上半：面试示意，纯噪声冠军会吹多高。
下半：同一条行情上，你的规则 vs 随机买卖 N 次的冠军。把 mkt、策略仓位、n_tried 换成你的。
"""

import numpy as np
import pandas as pd

rng = np.random.default_rng(8)
FEE = 0.001  # 单边 10bp，和 10 一样：佣金+滑点合成


def sharpe(r):
    r = np.asarray(r, dtype=float)
    return float(r.mean() / r.std(ddof=1) * np.sqrt(252))


def net_from_pos(pos, r, fee=FEE):
    # 昨仓 × 今收益 − |Δ仓|×单边成本。和 10 同一套，禁止 pos*r。
    pos = pd.Series(np.asarray(pos, dtype=float))
    r = pd.Series(np.asarray(r, dtype=float))
    return (pos.shift(1) * r - pos.diff().abs() * fee).dropna()


def n_flips(pos):
    return int(pd.Series(pos).diff().abs().fillna(0).gt(0).sum())


def random_pos_like(n, n_changes):
    # 随机多空，但翻仓次数≈你的策略。cuts 是随机换边的日子。
    pos = np.ones(n)
    sign = float(rng.choice([-1.0, 1.0]))
    if n_changes <= 0:
        return pos * sign
    cuts = np.sort(rng.choice(np.arange(1, n), size=min(n_changes, n - 1), replace=False))
    last = 0
    for c in list(cuts) + [n]:
        pos[last:c] = sign
        sign = -sign
        last = c
    return pos


def max_sharpe_noise(n_trials, n_days):
    # 面试示意：另造噪声收益，和真实行情无关。实际尺子用下面的随机买卖。
    best = -np.inf
    for _ in range(n_trials):
        best = max(best, sharpe(rng.normal(0, 0.01, n_days)))
    return float(best)


print("面试示意：无成本高斯噪声里只报冠军（3.43 不是你要追的目标）")
print("  试得越多，运气冠军越大——说明「搜 100 次拿出 Sharpe 3」可以全是运气")
print("  没扣费、行情也是假的，所以这个数当实盘门槛没有用；往下看随机买卖")
for n in (1, 20, 100):
    print(f"  试 {n:3d} 次，运气冠军 Sharpe ≈ {max_sharpe_noise(n, 252):.2f}")

# --- 实际尺子：同一条行情，随机买卖 ---
# mkt 换成你的标的日收益。演示用假行情。
n_days = 252
mkt = rng.normal(0, 0.01, n_days)  # TODO: 换成真实日收益
# 你的策略仓位（演示：昨涨今多的动量）。TODO: 换成你的 pos，+1/−1/0
my_pos = np.sign(pd.Series(mkt).shift(1).fillna(0).to_numpy())
my_sr = sharpe(net_from_pos(my_pos, mkt))

n_tried = 20  # TODO: 搜参 / 海选 / 扫高原试过的组数
flips = n_flips(my_pos)
best_rand = -np.inf
for _ in range(n_tried):
    best_rand = max(best_rand, sharpe(net_from_pos(random_pos_like(n_days, flips), mkt)))

print(f"\n同一条行情、同一套扣费、换手约 {flips} 次：")
print(f"  你的策略 Sharpe {my_sr:.2f}")
print(f"  随机买卖 {n_tried} 次里的冠军 {best_rand:.2f}  ← 这才是可用的门槛")
print("高于门槛：才值得继续怀疑。低于：还没赢过「瞎做同样多次」。")
print("真正拍板仍看持有集。口条：真实行情上随机买卖当尺子，不要另造一列噪声收益。")
