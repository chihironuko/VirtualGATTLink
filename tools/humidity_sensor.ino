#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>
#include <BLE2902.h>
#include <DHT.h>

#define SERVICE_UUID "12345678-1234-1234-1234-1234567890ab"
#define CHAR_UUID_TX "abcd1234-5678-90ab-cdef-1234567890ab"

#define DHTPIN 27
#define DHTTYPE DHT11

DHT dht(DHTPIN, DHTTYPE);

BLECharacteristic* txChar;
bool deviceConnected = false;

class ServerCallbacks : public BLEServerCallbacks {
  void onConnect(BLEServer* pServer) override {
    deviceConnected = true;
    Serial.println("Client Connected");
  }

  void onDisconnect(BLEServer* pServer) override {
    deviceConnected = false;
    Serial.println("Client Disconnected");

    delay(100);
    pServer->getAdvertising()->start();
    Serial.println("Advertising Restarted");
  }
};

void setup() {
  Serial.begin(115200);
  dht.begin();

  BLEDevice::init("ESP32_DHT11");
  BLEServer* server = BLEDevice::createServer();
  server->setCallbacks(new ServerCallbacks());

  BLEService* service = server->createService(SERVICE_UUID);

  txChar = service->createCharacteristic(
             CHAR_UUID_TX,
             BLECharacteristic::PROPERTY_NOTIFY
           );
  txChar->addDescriptor(new BLE2902());

  service->start();

  BLEAdvertising* adv = BLEDevice::getAdvertising();
  adv->addServiceUUID(SERVICE_UUID);
  adv->setScanResponse(true); // iOS対策
  adv->setMinPreferred(0x06);
  adv->setMinPreferred(0x12);
  adv->start();

  Serial.println("Waiting for client connection...");
}

void loop() {
  if (deviceConnected) {

    float humidity = dht.readHumidity();
    float temperature = dht.readTemperature(); // 摂氏
    float heatIndex = dht.computeHeatIndex(temperature, humidity, false);

    if (isnan(humidity) || isnan(temperature)) {
      Serial.println("DHT read failed");
      delay(1000);
      return;
    }

    char msg[64];
    sprintf(msg, "H:%.1f T:%.1f HI:%.1f", humidity, temperature, heatIndex);

    txChar->setValue((uint8_t*)msg, strlen(msg));
    txChar->notify();

    Serial.println(msg);

    delay(1000);  // 1秒ごと送信
  }
}
