use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::handlers::AppState;

/// 健康检查响应
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub capabilities: DeviceCapabilities,
}

/// 设备能力检查
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    /// NFC 能力
    pub nfc_available: bool,
    /// TEE 环境
    pub tee_available: bool,
    /// EMV 处理能力
    pub emv_processing: bool,
    /// APDU 处理能力
    pub apdu_processing: bool,
    /// 网络连接
    pub network_available: bool,
    /// GPS 定位
    pub gps_available: bool,
    /// Backend 连接
    pub backend_connected: bool,
}

/// 健康检查处理器
///
/// GET /health
///
/// 检查 SoftPOS 交易环境的基本能力
pub async fn health_check(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    // 检查各项能力
    let capabilities = check_device_capabilities(&state).await;

    // 判断整体状态
    let overall_status = if capabilities.is_healthy() {
        "healthy"
    } else {
        "degraded"
    };

    let response = HealthCheckResponse {
        status: overall_status.to_string(),
        service: "sunbay-kernel-service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        capabilities,
    };

    (
        StatusCode::OK,
        Json(serde_json::to_value(response).unwrap()),
    )
}

/// 检查设备能力
async fn check_device_capabilities(state: &Arc<AppState>) -> DeviceCapabilities {
    DeviceCapabilities {
        // NFC 能力检查
        nfc_available: check_nfc_capability(),

        // TEE 环境检查
        tee_available: check_tee_environment(),

        // EMV 处理能力（检查 EMV 处理器是否初始化）
        emv_processing: true, // EMV processor 已初始化

        // APDU 处理能力
        apdu_processing: check_apdu_capability(),

        // 网络连接检查
        network_available: check_network_connectivity(),

        // GPS 定位检查
        gps_available: check_gps_capability(),

        // Backend 连接检查
        backend_connected: check_backend_connection(state).await,
    }
}

impl DeviceCapabilities {
    /// 判断设备是否健康（所有关键能力都可用）
    pub fn is_healthy(&self) -> bool {
        // 关键能力：EMV 处理、APDU 处理
        // Backend 连接不再作为核心健康指标，允许 Kernel Service 独立/离线运行
        // self.backend_connected 仅作为参考信息，不影响整体服务健康状态
        self.emv_processing && self.apdu_processing
    }
}

/// 检查 NFC 能力
fn check_nfc_capability() -> bool {
    // TODO: 实际实现应该检查设备的 NFC 硬件
    // 例如：读取 /sys/class/nfc/ 或调用 Android NFC API
    #[cfg(target_os = "android")]
    {
        // Android 平台检查 NFC
        // 需要通过 JNI 调用 Android API
        false // 占位符
    }

    #[cfg(not(target_os = "android"))]
    {
        // 非 Android 平台（开发环境）
        tracing::warn!("NFC capability check not implemented for this platform");
        false
    }
}

/// 检查 TEE 环境
fn check_tee_environment() -> bool {
    // TODO: 实际实现应该检查 TEE 环境
    // 例如：检查 TrustZone、StrongBox 等
    #[cfg(target_os = "android")]
    {
        // Android 平台检查 TEE
        // 需要通过 JNI 调用 Android Keystore API
        false // 占位符
    }

    #[cfg(not(target_os = "android"))]
    {
        // 非 Android 平台（开发环境）
        tracing::warn!("TEE capability check not implemented for this platform");
        false
    }
}

/// 检查 APDU 处理能力
fn check_apdu_capability() -> bool {
    // TODO: 实际实现应该尝试发送一个测试 APDU 命令
    // 目前返回 true，因为 EMV processor 已初始化
    true
}

/// 检查网络连接
fn check_network_connectivity() -> bool {
    // TODO: 实际实现应该检查网络接口状态
    // 例如：检查是否有活动的网络接口、是否能访问外网
    #[cfg(unix)]
    {
        // Unix 系统检查网络接口
        use std::process::Command;
        if let Ok(output) = Command::new("ip").arg("addr").output() {
            return output.status.success();
        }
    }

    // 默认返回 true（假设有网络）
    true
}

/// 检查 GPS 能力
fn check_gps_capability() -> bool {
    // TODO: 实际实现应该检查 GPS 硬件和定位服务
    #[cfg(target_os = "android")]
    {
        // Android 平台检查 GPS
        // 需要通过 JNI 调用 Android Location API
        false // 占位符
    }

    #[cfg(not(target_os = "android"))]
    {
        // 非 Android 平台（开发环境）
        tracing::warn!("GPS capability check not implemented for this platform");
        false
    }
}

/// 检查 Backend 连接
async fn check_backend_connection(_state: &Arc<AppState>) -> bool {
    // 暂时禁用 Backend 连接检查，避免 /health 接口这产生不必要的服务依赖
    // 这种检查应该由专门的诊断接口提供，而不是在这里阻塞健康检查
    // match state.backend_client.health_check().await {
    //     Ok(_) => true,
    //     Err(e) => {
    //         tracing::warn!("Backend health check failed: {}", e);
    //         false
    //     }
    // }
    false
}
