// ESP32 I2C Slave using ESP-IDF driver (NOT Wire library)
// This is more reliable than Arduino Wire for slave mode

#include <driver/i2c.h>

// I2C Configuration
#define I2C_SLAVE_ADDR 0x08
#define I2C_SLAVE_SDA_IO 21
#define I2C_SLAVE_SCL_IO 22
#define I2C_SLAVE_NUM I2C_NUM_0
#define I2C_SLAVE_TX_BUF_LEN 256
#define I2C_SLAVE_RX_BUF_LEN 256

// Buffer for communication
uint8_t rxBuffer[128];
uint8_t txBuffer[128];
volatile int rxBytes = 0;
volatile bool dataReceived = false;

void setup() {
  Serial.begin(9600);
  delay(2000);
  
  Serial.println("\n=== ESP32 I2C Slave (ESP-IDF Driver) ===");
  Serial.print("Address: 0x");
  Serial.println(I2C_SLAVE_ADDR, HEX);
  Serial.print("SDA: GPIO ");
  Serial.println(I2C_SLAVE_SDA_IO);
  Serial.print("SCL: GPIO ");
  Serial.println(I2C_SLAVE_SCL_IO);
  
  // Configure I2C slave
  i2c_config_t conf_slave;
  conf_slave.sda_io_num = I2C_SLAVE_SDA_IO;
  conf_slave.sda_pullup_en = GPIO_PULLUP_ENABLE;
  conf_slave.scl_io_num = I2C_SLAVE_SCL_IO;
  conf_slave.scl_pullup_en = GPIO_PULLUP_ENABLE;
  conf_slave.mode = I2C_MODE_SLAVE;
  conf_slave.slave.addr_10bit_en = 0;
  conf_slave.slave.slave_addr = I2C_SLAVE_ADDR;
  conf_slave.clk_flags = 0;
  
  // Configure and install I2C driver
  esp_err_t err = i2c_param_config(I2C_SLAVE_NUM, &conf_slave);
  if (err != ESP_OK) {
    Serial.print("Config error: ");
    Serial.println(err);
    return;
  }
  
  err = i2c_driver_install(I2C_SLAVE_NUM, conf_slave.mode, 
                          I2C_SLAVE_RX_BUF_LEN, I2C_SLAVE_TX_BUF_LEN, 0);
  if (err != ESP_OK) {
    Serial.print("Driver install error: ");
    Serial.println(err);
    return;
  }
  
  Serial.println("I2C Slave ready!");
  Serial.println("Waiting for master...");
  Serial.println("================================\n");
  
  // Prepare a response in TX buffer
  txBuffer[0] = 0xAA;
}

void loop() {
  // Check for received data (non-blocking)
  int size = i2c_slave_read_buffer(I2C_SLAVE_NUM, rxBuffer, sizeof(rxBuffer), 100 / portTICK_PERIOD_MS);
  
  if (size > 0) {
    Serial.print(">>> RECEIVED ");
    Serial.print(size);
    Serial.print(" byte(s): ");
    
    for (int i = 0; i < size; i++) {
      Serial.print("0x");
      if (rxBuffer[i] < 16) Serial.print("0");
      Serial.print(rxBuffer[i], HEX);
      Serial.print(" ");
      
      // Echo back: prepare response = received byte + 1
      txBuffer[0] = rxBuffer[i] + 1;
    }
    Serial.println();
    
    // Write response to TX buffer (ready for next master read)
    i2c_slave_write_buffer(I2C_SLAVE_NUM, txBuffer, 1, 100 / portTICK_PERIOD_MS);
    Serial.print("<<< Prepared response: 0x");
    if (txBuffer[0] < 16) Serial.print("0");
    Serial.println(txBuffer[0], HEX);
  }
  
  // Heartbeat
  static unsigned long lastHeartbeat = 0;
  if (millis() - lastHeartbeat > 5000) {
    Serial.println("ESP32 alive and listening...");
    lastHeartbeat = millis();
  }
  
  delay(10);
}