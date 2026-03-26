use std::process::Command;

fn main() {
    let ps_script = r#"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new($false)
$devices = Get-PnpDevice -PresentOnly -Class Camera,Image |
    Select-Object FriendlyName,InstanceId,Status

if (-not $devices) { return }

$devices | ForEach-Object {
    "{0}`t{1}`t{2}" -f $_.FriendlyName, $_.InstanceId, $_.Status
}
"#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script])
        .output()
        .expect("PowerShell の起動に失敗しました。");

    if !output.status.success() {
        eprintln!(
            "PowerShell 実行エラー:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }

    let text = String::from_utf8_lossy(&output.stdout);

    if text.trim().is_empty() {
        println!("カメラデバイスは見つかりませんでした。");
        return;
    }

    for (index, line) in text.lines().enumerate() {
        let mut cols = line.split('\t');
        let friendly_name = cols.next().unwrap_or("(名前なし)");
        let instance_id = cols.next().unwrap_or("(IDなし)");
        let status = cols.next().unwrap_or("(状態なし)");

        println!("[{}] {}", index + 1, friendly_name);
        println!("    InstanceId: {}", instance_id);
        println!("    Status: {}", status);
    }
}
