## コード内訳  
* humidity_sensor.ino  
esp32に接続したDHT11のデータを送信する単純なコード  
notifyでのデータ送信  

* bluetooth_receiver.py
esp32のデータをbleで受け取るコード  
raspberry piでの動作を想定しているが、基本bluetoothが使えれば問題ない  
sensorと直に接続しているように見せかけることが目標なので、細かい部分は何でも良い  