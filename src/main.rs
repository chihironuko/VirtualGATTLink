use nokhwa::query;
use nokhwa::utils::ApiBackend;

fn main() {
    let camera_infos = match query(ApiBackend::Auto) {
        Ok(list) => list,
        Err(error) => {
            eprintln!("カメラ一覧の取得に失敗しました: {error}");
            return;
        }
    };

    if camera_infos.is_empty() {
        println!("カメラは見つかりませんでした。");
        return;
    }

    // 「USB接続らしい」デバイスを、取得情報の文字列から簡易判定
    let usb_cameras: Vec<_> = camera_infos
        .iter()
        .filter(|info| format!("{info:?}").to_lowercase().contains("usb"))
        .collect();

    if usb_cameras.is_empty() {
        println!("USBと判定できるカメラは見つかりませんでした。");
        println!("取得できたカメラ一覧:");
        for (index, info) in camera_infos.iter().enumerate() {
            println!("[{}] {:?}", index + 1, info);
        }
        return;
    }

    println!("USB接続カメラ候補: {}", usb_cameras.len());
    for (index, info) in usb_cameras.iter().enumerate() {
        println!("[{}] {:?}", index + 1, info);
    }
}
