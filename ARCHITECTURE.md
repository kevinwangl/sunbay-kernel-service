# SUNBAY Kernel Service Architecture

## Overview

The Kernel Service is a lightweight, stateless microservice focused on EMV transaction processing and SoftPOS attestation. It acts as a bridge between payment devices and the backend system.

## System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Payment Device (NFC/Card)               │
└─────────────────────┬───────────────────────────────────┘
                      │ Card Data
                      ↓
┌─────────────────────────────────────────────────────────┐
│              Kernel Service (本服务)                     │
│  ┌─────────────────────────────────────────────────┐   │
│  │  EMV Processor                                   │   │
│  │  - APDU Commands  - TLV Parsing                 │   │
│  │  - Card Selection - Data Extraction             │   │
│  └─────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Backend Client                                  │   │
│  │  - Transaction Attestation                      │   │
│  │  - HTTP Communication                           │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────┬───────────────────────────────────┘
                      │ Attestation Request
                      ↓
┌─────────────────────────────────────────────────────────┐
│              SoftPOS Backend                             │
│  - Device Validation                                    │
│  - Transaction Storage                                  │
│  - Key Management                                       │
│  - Kernel Management                                    │
└─────────────────────────────────────────────────────────┘
```

## Technology Stack

- **Language**: Rust
- **Web Framework**: Axum
- **No Database**: Stateless service, all data stored in backend
- **HTTP Client**: Reqwest
- **EMV Processing**: Custom APDU and TLV parsers

## Core Responsibilities

### 1. EMV Card Interaction
- **APDU Command Processing**: SELECT, READ RECORD, GPO, GENERATE AC
- **TLV Data Parsing**: Extract card data from EMV responses
- **Card Data Extraction**: PAN, expiry, track data, cryptograms

### 2. Transaction Attestation
- **Data Aggregation**: Collect card data and transaction details
- **Backend Communication**: Forward attestation requests to backend
- **Response Handling**: Return attestation results to device

## Data Flow

### EMV Transaction Flow

```
Device → POST /api/emv/select (AID)
    ↓
Kernel Service: Build SELECT command
    ↓
Return: FCI data
    ↓
Device → POST /api/emv/read (SFI, Record)
    ↓
Kernel Service: Build READ RECORD command
    ↓
Return: Card data (TLV)
    ↓
Device → POST /api/emv/gpo (PDOL)
    ↓
Kernel Service: Build GPO command
    ↓
Return: AIP, AFL
    ↓
Device → POST /api/transactions/attest
    ↓
Kernel Service: Forward to Backend
    ↓
Backend: Validate and store transaction
    ↓
Return: Transaction ID, Status
```

## API Endpoints

### EMV Card Interaction
- `POST /api/emv/select` - Select application by AID
- `POST /api/emv/read` - Read record from card
- `POST /api/emv/gpo` - Get processing options

### Transaction Management
- `POST /api/transactions/attest` - Attest transaction
- `GET /api/transactions/:id/status` - Get transaction status

### Health Check
- `GET /health` - Service health status

## Security Design

### 1. Stateless Architecture
- No local data storage
- No session management
- All state managed by backend

### 2. Backend Integration
- HTTPS communication with backend
- Transaction data forwarded securely
- Device validation delegated to backend

### 3. EMV Compliance
- Standard APDU command processing
- TLV data parsing per EMV specifications
- Cryptogram handling (delegated to backend)

## Performance Characteristics

### Latency
- **EMV Commands**: < 100ms (local processing)
- **Attestation**: < 500ms (includes backend roundtrip)

### Scalability
- **Stateless**: Easy horizontal scaling
- **No Database**: No connection pool limits
- **Lightweight**: Minimal resource usage

### Concurrency
- Tokio async runtime
- Non-blocking I/O
- Handles multiple concurrent requests

## Deployment

### Requirements
- No database required
- Backend URL configuration
- Minimal memory footprint (~50MB)

### Configuration
```yaml
server:
  host: 0.0.0.0
  port: 3000

backend:
  url: "http://backend:8080"

logging:
  level: info
```

## Monitoring

### Key Metrics
- EMV command processing time
- Backend communication latency
- Error rates by endpoint
- Request throughput

### Health Checks
- Service availability: `GET /health`
- Backend connectivity: Automatic health check

## Comparison with Previous Architecture

| Feature | Old (Kernel Manager) | New (EMV Processor) |
|---------|---------------------|---------------------|
| Database | SQLite | None |
| Storage | Local files | None |
| Responsibility | Kernel hosting | EMV processing |
| State | Stateful | Stateless |
| Complexity | High | Low |
| Scalability | Limited | High |

## Future Enhancements

### Phase 1 (Current)
- ✅ EMV APDU processing
- ✅ Transaction attestation
- ✅ Backend integration

### Phase 2 (Planned)
- ⏳ Real card reader integration
- ⏳ Enhanced error handling
- ⏳ Performance optimization
- ⏳ Comprehensive logging

### Phase 3 (Future)
- ⏳ Contactless payment support
- ⏳ Multi-currency support
- ⏳ Advanced EMV features
- ⏳ Offline transaction support
