# SUNBAY Kernel Service

EMV 交易处理和 SoftPOS 鉴证服务（无状态架构）

## 功能

此服务专注于：
- ✅ EMV 卡交互（APDU 命令处理）
- ✅ 交易鉴证（转发到 backend）
- ✅ 健康检查
- ✅ 无数据库，完全无状态

> **架构说明**: 
> - 本服务不存储任何数据，所有数据由 `sunbay-softpos-backend` 管理
> - 专注于 EMV 协议处理和交易鉴证转发
> - 轻量级、易扩展、高性能

## 快速开始

```bash
# 启动服务
cargo run

# 健康检查
curl http://localhost:3000/health
```

## 配置

编辑 `config/development.yaml`:

```yaml
server:
  host: 127.0.0.1
  port: 3000

# Backend URL（必需）
backend:
  url: "http://127.0.0.1:8080"

# HMAC 签名密钥（可选，用于未来功能）
signing_key: "dev-secret-key-change-in-production"

# 缓存配置（保留用于未来功能）
cache:
  enabled: false
  max_size_mb: 0

# 日志级别
logging:
  level: debug
```

## API 文档

### 健康检查
```bash
GET /health
Response: { "status": "healthy", "service": "sunbay-kernel-service", "version": "..." }
```

### EMV 卡交互

#### 选择应用
```bash
POST /api/emv/select
Content-Type: application/json

{
  "aid": "A0000000031010",
  "device_id": "device-123"
}

Response:
{
  "success": true,
  "fci": "6F1A840E315041592E5359532E4444463031...",
  "sw": "9000"
}
```

#### 读取记录
```bash
POST /api/emv/read
Content-Type: application/json

{
  "sfi": 1,
  "record": 1,
  "device_id": "device-123"
}

Response:
{
  "success": true,
  "data": "70105A0812345678901234...",
  "sw": "9000"
}
```

#### 获取处理选项
```bash
POST /api/emv/gpo
Content-Type: application/json

{
  "pdol": "8300",
  "device_id": "device-123"
}

Response:
{
  "success": true,
  "aip": "1980",
  "afl": "0801010010010100180102",
  "sw": "9000"
}
```

### 交易鉴证

```bash
POST /api/transactions/attest
Content-Type: application/json

{
  "device_id": "device-123",
  "amount": 10000,
  "currency": "CNY",
  "card_data": {
    "pan": "1234****5678",
    "expiry": "2512",
    "aid": "A0000000031010"
  },
  "emv_data": {
    "amount": 10000,
    "currency_code": "CNY",
    "transaction_type": "purchase"
  }
}

Response:
{
  "transaction_id": "txn-456",
  "status": "approved",
  "auth_code": "123456"
}
```

## 与主 Backend 集成

此服务与 `sunbay-softpos-backend` 协同工作：

| 服务 | 职责 |
|------|------|
| **Backend** | 设备管理、密钥管理、版本管理、内核管理、数据存储 |
| **Kernel Service** | EMV 卡交互、APDU 处理、交易鉴证转发 |

### 数据流

```
设备 → Kernel Service (EMV处理) → Backend (验证/存储)
```

## 技术栈

- **语言**: Rust
- **Web 框架**: Axum
- **HTTP 客户端**: Reqwest
- **EMV 处理**: 自定义 APDU 和 TLV 解析器
- **数据库**: 无（完全无状态）

## 部署

### 编译
```bash
cargo build --release
```

### 运行
```bash
./target/release/sunbay-kernel-service
```

### Docker（可选）
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/sunbay-kernel-service /usr/local/bin/
CMD ["sunbay-kernel-service"]
```

## 性能特性

- **无状态**: 易于水平扩展
- **轻量级**: 内存占用 ~50MB
- **低延迟**: EMV 命令处理 < 100ms
- **高并发**: Tokio 异步运行时

## 监控

建议监控以下指标：
- EMV 命令处理时间
- Backend 通信延迟
- 错误率（按端点）
- 请求吞吐量

## 开发

### 运行测试
```bash
cargo test
```

### 代码检查
```bash
cargo check
cargo clippy
```

### 格式化
```bash
cargo fmt
```

## 许可证

Copyright © 2024 SUNBAY
