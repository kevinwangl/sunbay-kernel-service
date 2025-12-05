# API 文档

## 基础信息

- **Base URL**: `http://localhost:3000`
- **所有响应格式**: JSON
- **CORS**: 已启用
- **服务职责**: EMV 卡交互和交易鉴证（不包含内核管理）

> **重要**: 内核管理功能（上传、下载、版本管理）已迁移至 `sunbay-softpos-backend`。
> 本服务仅负责 EMV 交易处理和鉴证转发。

---

## 端点

### 1. 健康检查（设备能力检测）

```http
GET /health
```

**功能说明:**
检查 SoftPOS 交易环境的基本能力，包括：
- **NFC 能力**: 非接触式读卡功能
- **TEE 环境**: 可信执行环境（TrustZone/StrongBox）
- **EMV 处理**: EMV 卡片处理能力
- **APDU 处理**: APDU 命令收发能力
- **网络连接**: IP 网络可用性
- **GPS 定位**: 位置服务可用性
- **Backend 连接**: 后端服务连接状态

**响应:**
```json
{
  "status": "healthy",
  "service": "sunbay-kernel-service",
  "version": "0.1.0",
  "capabilities": {
    "nfc_available": true,
    "tee_available": true,
    "emv_processing": true,
    "apdu_processing": true,
    "network_available": true,
    "gps_available": true,
    "backend_connected": true
  }
}
```

**状态说明:**
- `healthy`: 所有关键能力（EMV、APDU、Backend）都可用
- `degraded`: 部分能力不可用，但核心功能仍可运行

**示例:**
```bash
curl http://localhost:3000/health
```

**使用场景:**
1. **设备注册前检查**: 验证设备是否满足 SoftPOS 要求
2. **交易前验证**: 确认设备环境具备交易能力
3. **监控告警**: 定期检查设备状态，发现异常及时告警
4. **故障诊断**: 快速定位设备能力问题


---

## EMV 卡交互 API

### 2. 选择应用 (SELECT)

```http
POST /api/emv/select
Content-Type: application/json
```

**请求体:**
```json
{
  "aid": "A0000000031010",
  "device_id": "device-123"
}
```

**参数说明:**
- `aid` (string): 应用标识符（AID），十六进制字符串
- `device_id` (string): 设备 ID

**响应:**
```json
{
  "success": true,
  "fci": "6F1A840E315041592E5359532E4444463031A5088801025F2D02656E",
  "sw": "9000"
}
```

**响应字段:**
- `success` (boolean): 操作是否成功
- `fci` (string): 文件控制信息（FCI），十六进制字符串
- `sw` (string): 状态字（Status Word），十六进制

**示例:**
```bash
curl -X POST http://localhost:3000/api/emv/select \
  -H "Content-Type: application/json" \
  -d '{"aid": "A0000000031010", "device_id": "device-123"}'
```

---

### 3. 读取记录 (READ RECORD)

```http
POST /api/emv/read
Content-Type: application/json
```

**请求体:**
```json
{
  "sfi": 1,
  "record": 1,
  "device_id": "device-123"
}
```

**参数说明:**
- `sfi` (number): 短文件标识符（Short File Identifier）
- `record` (number): 记录号
- `device_id` (string): 设备 ID

**响应:**
```json
{
  "success": true,
  "data": "70105A0812345678901234565F24031912",
  "sw": "9000"
}
```

**响应字段:**
- `success` (boolean): 操作是否成功
- `data` (string): 记录数据（TLV 格式），十六进制字符串
- `sw` (string): 状态字

**示例:**
```bash
curl -X POST http://localhost:3000/api/emv/read \
  -H "Content-Type: application/json" \
  -d '{"sfi": 1, "record": 1, "device_id": "device-123"}'
```

---

### 4. 获取处理选项 (GPO)

```http
POST /api/emv/gpo
Content-Type: application/json
```

**请求体:**
```json
{
  "pdol": "8300",
  "device_id": "device-123"
}
```

**参数说明:**
- `pdol` (string): 处理选项数据对象列表（PDOL）数据，十六进制字符串
- `device_id` (string): 设备 ID

**响应:**
```json
{
  "success": true,
  "aip": "1980",
  "afl": "0801010010010100180102",
  "sw": "9000"
}
```

**响应字段:**
- `success` (boolean): 操作是否成功
- `aip` (string): 应用交互特征（Application Interchange Profile）
- `afl` (string): 应用文件定位器（Application File Locator）
- `sw` (string): 状态字

**示例:**
```bash
curl -X POST http://localhost:3000/api/emv/gpo \
  -H "Content-Type: application/json" \
  -d '{"pdol": "8300", "device_id": "device-123"}'
```

---

## 交易鉴证 API

### 5. 交易鉴证

```http
POST /api/transactions/attest
Content-Type: application/json
```

**请求体:**
```json
{
  "device_id": "device-123",
  "amount": 10000,
  "currency": "CNY",
  "card_data": {
    "pan": "1234****5678",
    "expiry": "2512",
    "cardholder_name": "JOHN DOE",
    "track2": "1234567890123456D25121011234567890",
    "aid": "A0000000031010",
    "app_label": "VISA CREDIT"
  },
  "emv_data": {
    "amount": 10000,
    "currency_code": "CNY",
    "transaction_type": "purchase",
    "tvr": "0000000000",
    "tsi": "E800",
    "cryptogram": "1234567890ABCDEF",
    "cid": 128
  }
}
```

**参数说明:**
- `device_id` (string): 设备 ID
- `amount` (number): 交易金额（分）
- `currency` (string): 货币代码
- `card_data` (object): 卡片数据
  - `pan` (string): 主账号（已脱敏）
  - `expiry` (string): 有效期（YYMM）
  - `cardholder_name` (string, optional): 持卡人姓名
  - `track2` (string, optional): 磁道 2 数据
  - `aid` (string): 应用标识符
  - `app_label` (string, optional): 应用标签
- `emv_data` (object): EMV 交易数据
  - `amount` (number): 交易金额
  - `currency_code` (string): 货币代码
  - `transaction_type` (string): 交易类型（purchase/withdrawal/refund/cashadvance）
  - `tvr` (string, optional): 终端验证结果
  - `tsi` (string, optional): 交易状态信息
  - `cryptogram` (string, optional): 应用密文
  - `cid` (number, optional): 密文信息数据

**响应:**
```json
{
  "transaction_id": "txn-456",
  "status": "approved",
  "auth_code": "123456",
  "message": "Transaction approved"
}
```

**响应字段:**
- `transaction_id` (string): 交易 ID
- `status` (string): 交易状态（approved/declined/failed）
- `auth_code` (string, optional): 授权码
- `message` (string, optional): 响应消息

**示例:**
```bash
curl -X POST http://localhost:3000/api/transactions/attest \
  -H "Content-Type: application/json" \
  -d '{
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
  }'
```

---

### 6. 查询交易状态

```http
GET /api/transactions/:id/status
```

**路径参数:**
- `id` (string): 交易 ID

**响应:**
```json
{
  "transaction_id": "txn-456",
  "status": "approved"
}
```

**示例:**
```bash
curl http://localhost:3000/api/transactions/txn-456/status
```

---

## 错误响应

所有错误响应格式:

```json
{
  "error": "错误信息"
}
```

**状态码:**
- `400 Bad Request`: 请求参数错误
- `404 Not Found`: 资源不存在
- `500 Internal Server Error`: 服务器内部错误

---

## 状态字说明

EMV 状态字（SW1-SW2）常见值：

| 状态字 | 含义 |
|--------|------|
| `9000` | 成功 |
| `6283` | 选中的文件已失效 |
| `6700` | 错误的长度 |
| `6982` | 安全状态不满足 |
| `6985` | 不满足使用条件 |
| `6A81` | 功能不支持 |
| `6A82` | 文件未找到 |
| `6A83` | 记录未找到 |

---

## 集成示例

### JavaScript/TypeScript

```typescript
class EmvProcessor {
    private baseUrl = 'http://localhost:3000';
    
    async selectApplication(aid: string, deviceId: string) {
        const response = await fetch(`${this.baseUrl}/api/emv/select`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ aid, device_id: deviceId })
        });
        return await response.json();
    }
    
    async readRecord(sfi: number, record: number, deviceId: string) {
        const response = await fetch(`${this.baseUrl}/api/emv/read`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ sfi, record, device_id: deviceId })
        });
        return await response.json();
    }
    
    async attestTransaction(transactionData: any) {
        const response = await fetch(`${this.baseUrl}/api/transactions/attest`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(transactionData)
        });
        return await response.json();
    }
}
```

### Python

```python
import requests

class EmvClient:
    def __init__(self, base_url='http://localhost:3000'):
        self.base_url = base_url
    
    def select_application(self, aid: str, device_id: str):
        response = requests.post(
            f'{self.base_url}/api/emv/select',
            json={'aid': aid, 'device_id': device_id}
        )
        return response.json()
    
    def attest_transaction(self, transaction_data: dict):
        response = requests.post(
            f'{self.base_url}/api/transactions/attest',
            json=transaction_data
        )
        return response.json()
```

---

## 与 Backend 集成

本服务与 `sunbay-softpos-backend` 协同工作：

```
设备 → Kernel Service → Backend
      (EMV 处理)      (验证/存储)
```

**Kernel Service 负责:**
- EMV APDU 命令处理
- 卡片数据提取
- 交易鉴证转发

**Backend 负责:**
- 设备验证
- 交易存储
- 密钥管理
- 内核版本管理

---

## 注意事项

1. **无状态服务**: 本服务不存储任何数据，所有数据由 backend 管理
2. **模拟响应**: 当前实现使用模拟的 APDU 响应，实际部署需要集成硬件读卡器
3. **安全性**: 生产环境应使用 HTTPS 和适当的认证机制
4. **内核管理**: 内核的上传、下载、版本管理请使用 `sunbay-softpos-backend` 的 API
