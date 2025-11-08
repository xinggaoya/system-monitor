use crate::errors::MonitorError;
#[cfg(target_os = "windows")]
use crate::models::FrameDataSource;
use crate::models::FrameStats;
#[cfg(target_os = "windows")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use std::process::{Command, Stdio};
use std::time::Duration;
#[cfg(target_os = "windows")]
use std::time::SystemTime;

#[derive(Clone)]
pub struct FrameMonitor {
    #[cfg(target_os = "windows")]
    presentmon_path: Option<PathBuf>,
}

impl FrameMonitor {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            presentmon_path: Self::locate_presentmon(),
        }
    }

    #[allow(dead_code)]
    pub fn is_available(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            self.presentmon_path.is_some()
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    pub async fn capture_frame_stats(
        &self,
        duration: Duration,
    ) -> Result<FrameStats, MonitorError> {
        #[cfg(target_os = "windows")]
        {
            self.capture_with_presentmon(duration).await
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = duration;
            Err(MonitorError::GenericError(
                "当前平台暂不支持帧率采集".to_string(),
            ))
        }
    }

    #[cfg(target_os = "windows")]
    fn locate_presentmon() -> Option<PathBuf> {
        if let Some(explicit) = std::env::var_os("PRESENTMON_PATH") {
            let path = PathBuf::from(explicit);
            if path.exists() {
                return Some(path);
            }
        }

        if let Ok(path_var) = std::env::var("PATH") {
            for candidate in path_var.split(';') {
                let candidate_path = Path::new(candidate).join("PresentMon.exe");
                if candidate_path.exists() {
                    return Some(candidate_path);
                }
            }
        }

        None
    }

    #[cfg(target_os = "windows")]
    async fn capture_with_presentmon(
        &self,
        duration: Duration,
    ) -> Result<FrameStats, MonitorError> {
        use tokio::task::spawn_blocking;

        let presentmon = self.presentmon_path.clone().ok_or_else(|| {
            MonitorError::ConfigError(
                "未找到 PresentMon.exe，请安装 Intel PresentMon 并配置 PRESENTMON_PATH 环境变量"
                    .into(),
            )
        })?;

        let duration_secs = duration.as_secs().clamp(1, 5);
        let args = vec![
            "-captureall".to_string(),
            "-simple".to_string(),
            "-no_csv".to_string(),
            "-output_stdout".to_string(),
            "-timed".to_string(),
            duration_secs.to_string(),
            "-exclude_dropped".to_string(),
        ];

        let output = spawn_blocking(move || {
            Command::new(&presentmon)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        })
        .await
        .map_err(|err| MonitorError::GenericError(format!("帧率采集线程失败: {err}")))?
        .map_err(|err| MonitorError::IoError(format!("执行 PresentMon 失败: {err}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(MonitorError::GenericError(format!(
                "PresentMon 退出异常: {stderr}"
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut fps_total = 0.0f32;
        let mut sample_count = 0u32;

        for line in stdout.lines() {
            if line.trim().is_empty() || line.contains("ProcessName") || line.contains("ProcessID")
            {
                continue;
            }
            if let Some(fps_str) = line.split(',').last() {
                if let Ok(fps) = fps_str.trim().parse::<f32>() {
                    if fps.is_finite() && fps >= 0.0 {
                        fps_total += fps;
                        sample_count += 1;
                    }
                }
            }
        }

        if sample_count == 0 {
            return Err(MonitorError::GenericError(
                "PresentMon 未产生有效帧率数据".into(),
            ));
        }

        let average_fps = fps_total / sample_count as f32;
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Ok(FrameStats {
            average_fps,
            sample_count,
            duration_ms: duration_secs * 1000,
            timestamp,
            source: FrameDataSource::PresentMon,
        })
    }
}
