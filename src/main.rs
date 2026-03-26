use nokhwa::query;
use nokhwa::utils::ApiBackend;

// すでにカメラ専用のエージェントに見えてきた
// 外部通信を作成して、通信先に自身が管理できるカメラ一覧を返すとかできそう
fn detect_nokhwa() -> Result<(), String> {
    // API設定(OS選択)
    let camera_infos = query(ApiBackend::Auto).map_err(|e| format!("nokhwa列挙失敗: {e}"))?;

    if camera_infos.is_empty() {
        println!("[nokhwa] カメラは見つかりませんでした。");
        return Ok(());
    }

    println!("[nokhwa] 検出カメラ数: {}", camera_infos.len());
    for (i, info) in camera_infos.iter().enumerate() {
        println!("[nokhwa][{}] {:?}", i + 1, info);
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn detect_libcamera() -> Result<(), String> {
    use libcamera::camera_manager::CameraManager;

    let mgr = CameraManager::new().map_err(|e| format!("libcamera初期化失敗: {e}"))?;
    let cameras = mgr.cameras();

    if cameras.is_empty() {
        println!("[libcamera] カメラは見つかりませんでした。");
        return Ok(());
    }

    println!("[libcamera] 検出カメラ数: {}", cameras.len());
    for (i, cam) in cameras.iter().enumerate() {
        println!("[libcamera][{}] ID: {}", i + 1, cam.id());
        println!("[libcamera][{}] Properties: {:#?}", i + 1, cam.properties());
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn detect_libcamera() -> Result<(), String> {
    println!("[libcamera] このOSではスキップします。");
    Ok(())
}

fn main() {
    println!("=== Camera Detection Start ===");

    if let Err(e) = detect_nokhwa() {
        eprintln!("{e}");
    }

    if let Err(e) = detect_libcamera() {
        eprintln!("{e}");
    }

    println!("=== Camera Detection End ===");
}
