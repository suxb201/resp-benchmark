# resp-benchmark

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/your_username/resp-benchmark/blob/main/LICENSE)

用来测试兼容 RESP 协议的数据库，比如 Redis、Valkey、Tair 等，提供命令行工具与 Python 库（方便编写自动化测试脚本）。

## 安装

```bash
pip install resp-benchmark
```

## 使用

### 命令行工具

```bash
resp-benchmark --help
```

### Python 库

```python
from resp_benchmark import Benchmark

bm = Benchmark(host="127.0.0.1", port=6379)
bm.flushall()
bm.load_data(command="SET {key sequence 10000000} {value 64}", count=1000_0000, connections=128)
result = bm.bench("GET {key uniform 10000000}", seconds=3, connections=16)
print(result.qps, result.avg_latency_ms, result.p99_latency_ms)
```

## 自定义命令

resp-benchmark 支持自定义要测试的命令，使用如下占位符语法：`SET {key uniform 10000000} {value 64}` 表示执行 SET 命令，key 的分布是 uniform，随机范围是 0-10000000，value 的大小是 64 字节

支持的占位符有：
- **`{key uniform N}`**: 生成范围在 `0` 至 `N-1` 的随机数。比如 `{key uniform 100}` 可能会生成 `key_0000000099`。
- **`{key sequence N}`**: 同上，但是是顺序产生，用于在加载数据时保证数据覆盖。比如 `{key sequence 100}` 会生成 `key_0000000000`, `key_0000000001`, ...
- **`{key zipfian N}`**: 同上，但是分布是指数为 1.03 的 Zipfian 分布，用于模拟真实场景下的 key 分布。
- **`{value N}`**: 生成长度为 `N` 字节的随机字符串。比如 `{value 64}` 可能会生成 `92xsqdNgAyKcqtR4pyXz7j1GQAlRJQJ9TagOmCZ5xR3q3UCXl6B7QysZfgYd4Vmd`。
- **`{rand N}`**: 生成一个 `0` 到 `N-1` 之间的随机数。比如 `{rand 100}` 可能会生成 `99`。
- **`{range N W}`**: 生成一对随机数，两个数字的范围是 `0` 到 `N-1`，两个数字的差值是 `W`，用来测试各类 `*range*` 命令。比如 `{range 100 10}` 可能会生成 `89 99`。

## 最佳实践


### 压测 zset 

```shell
# 1. 加载数据
resp-benchmark --load -n 1000000 -P 10 "ZADD {key sequence 1000} {rand 1000} {value 8}"
# 2. 压测
resp-benchmark "ZRANGEBYSCORE {key uniform 1000} {range 1000 10}"
```

### 压测 lua script

```shell
redis-cli 'SCRIPT LOAD "return redis.call('\''SET'\'', KEYS[1], ARGV[1])"'
resp-benchmark "EVALSHA d8f2fad9f8e86a53d2a6ebd960b33c4972cacc37 1 {key uniform 100000} {value 64}"
```

## 与 redis-benchmark 的差异

使用 resp-benchmark 与 redis-benchmark 测试 Redis 时，可能会得到不同的结果。常见有以下原因：
1. redis-benchmark 在测试 set 命令时总是使用相同的 value，这不会导致 DB 的持久化与复制机制。resp-benchmark 则可以使用 `{value 64}` 对每条命令都重新生成数据。
2. redis-benchmark 在测试 list/set/zset/hash 等命令时，总是使用相同的 primary key，可能会导致多线程 DB 的性能数据失真。resp-benchmark 可以通过 `{key uniform 10000000}` 等占位符生成不同的 key。
3. redis-benchmark 在集群模式下，发到不同节点的请求都被指定了相同的 slot，可能会导致多线程 DB 的性能数据失真。