// ESP32 I2C Slave - Beginner Friendly Version
// This makes I2C slave communication simple and easy to understand

#include <driver/i2c.h>

// ===== CONFIGURATION - Change these if needed =====
#define I2C_ADDRESS 0x08        // ESP32's address on I2C bus (like a house number)
#define SDA_PIN 21              // Data pin
#define SCL_PIN 22              // Clock pin
#define BUTTON_PIN 14           // Push button

// ===== Internal buffers (you don't need to touch these) =====
#define BUFFER_SIZE 128
uint8_t receiveBuffer[BUFFER_SIZE];
uint8_t sendBuffer[BUFFER_SIZE];
int sendBufferLength = 0;
// This byte holds the message for the Pi.
// 0x00 = No event
// 0x01 = Button was pressed
volatile uint8_t messageForPi = 0x00;

// ===== SETUP FUNCTION - Call this once in setup() =====
bool initI2CSlave() {
  // Configure I2C hardware
  i2c_config_t conf;
  conf.sda_io_num = SDA_PIN;
  conf.sda_pullup_en = GPIO_PULLUP_ENABLE;
  conf.scl_io_num = SCL_PIN;
  conf.scl_pullup_en = GPIO_PULLUP_ENABLE;
  conf.mode = I2C_MODE_SLAVE;
  conf.slave.addr_10bit_en = 0;
  conf.slave.slave_addr = I2C_ADDRESS;
  conf.clk_flags = 0;
  
  // Install I2C driver
  if (i2c_param_config(I2C_NUM_0, &conf) != ESP_OK) {
    return false;
  }
  if (i2c_driver_install(I2C_NUM_0, conf.mode, BUFFER_SIZE, BUFFER_SIZE, 0) != ESP_OK) {
    return false;
  }
  
  return true;
}

// ===== CHECK FOR NEW DATA - Returns number of bytes received =====
int checkForData() {
  return i2c_slave_read_buffer(I2C_NUM_0, receiveBuffer, BUFFER_SIZE, 10 / portTICK_PERIOD_MS);
}

// ===== GET RECEIVED DATA - Get a specific byte from received data =====
uint8_t getReceivedByte(int index) {
  if (index < BUFFER_SIZE) {
    return receiveBuffer[index];
  }
  return 0;
}

// ===== SEND DATA BACK - Prepare data to send when Pi requests it =====
void sendData(uint8_t* data, int length) {
  if (length > BUFFER_SIZE) {
    length = BUFFER_SIZE;
  }
  
  for (int i = 0; i < length; i++) {
    sendBuffer[i] = data[i];
  }
  sendBufferLength = length;
  
  i2c_slave_write_buffer(I2C_NUM_0, sendBuffer, length, 100 / portTICK_PERIOD_MS);
}

// ===== SEND A SINGLE BYTE - Simple version for sending one byte =====
void sendByte(uint8_t value) {
  sendBuffer[0] = value;
  sendBufferLength = 1;
  i2c_slave_write_buffer(I2C_NUM_0, sendBuffer, 1, 100 / portTICK_PERIOD_MS);
}

// ============================================================
// YOUR CODE STARTS HERE - This is where you write your logic
// ============================================================

void setup() {
  Serial.begin(9600);
  delay(2000);

  Serial.println("\n=== ESP32 I2C Slave with Button ===");
  
  // Configure the button pin
  pinMode(BUTTON_PIN, INPUT_PULLUP);
  Serial.print("✓ Button configured on GPIO ");
  Serial.println(BUTTON_PIN);
  Serial.print("Address: 0x");
  Serial.println(I2C_ADDRESS, HEX);
  
  // Initialize I2C slave
  if (initI2CSlave()) {
    Serial.println("✓ I2C Slave initialized successfully!");
  } else {
    Serial.println("✗ I2C initialization failed!");
    while(1) delay(1000);  // Stop here if failed
  }

  Serial.println("Waiting for commands and button presses...\n");
}

void loop() {
  static int lastButtonState = HIGH; 

  int currentButtonState = digitalRead(BUTTON_PIN);
  
  if (lastButtonState == HIGH && currentButtonState == LOW) {
    Serial.println("--- Button Pressed! ---");
    Serial.println("Setting message for Pi to 0x01\n");
    messageForPi = 0x01;
    delay(50); 
  }
  
  lastButtonState = currentButtonState;
  
  // 1. Create a normal, temporary copy of the message.
  uint8_t messageToSend = messageForPi;

  // 2. Pass the temporary copy to the function. This is now a valid type conversion.
  i2c_slave_write_buffer(I2C_NUM_0, &messageToSend, 1, 0); 

  if (messageForPi == 0x01) {
    messageForPi = 0x00;
  }

  delay(10);
}



// Old Program
/*
void loop() {
  // Check if Pi sent us any data
  int bytesReceived = checkForData();
  
  if (bytesReceived > 0) {
    Serial.println("--- Data Received ---");
    Serial.print("Bytes: ");
    Serial.println(bytesReceived);
    
    // Print all received bytes
    Serial.print("Data: ");
    for (int i = 0; i < bytesReceived; i++) {
      uint8_t receivedByte = getReceivedByte(i);
      Serial.print("0x");
      if (receivedByte < 16) Serial.print("0");
      Serial.print(receivedByte, HEX);
      Serial.print(" ");
    }
    Serial.println();
    
    // ==== YOUR CUSTOM LOGIC HERE ====
    // Example: Echo back the first byte + 1
    uint8_t firstByte = getReceivedByte(0);
    uint8_t response = firstByte + 1;
    
    sendByte(response);
    Serial.print("Sent response: 0x");
    if (response < 16) Serial.print("0");
    Serial.println(response, HEX);
    Serial.println();
  }
  
  delay(10);  // Small delay to prevent overwhelming the CPU
}
*/

// ============================================================
// EXAMPLE USAGE PATTERNS
// ============================================================

/*
// EXAMPLE 1: Simple echo - send back what you received
void loop() {
  int bytes = checkForData();
  if (bytes > 0) {
    uint8_t data = getReceivedByte(0);
    sendByte(data);  // Echo it back
  }
  delay(10);
}

// EXAMPLE 2: Command-based system
void loop() {
  int bytes = checkForData();
  if (bytes > 0) {
    uint8_t command = getReceivedByte(0);
    
    if (command == 0x01) {
      // Command 1: Return temperature (example)
      sendByte(25);  // Send 25 degrees
    } 
    else if (command == 0x02) {
      // Command 2: Return status
      sendByte(0xFF);  // Send "OK" status
    }
  }
  delay(10);
}

// EXAMPLE 3: Send multiple bytes back
void loop() {
  int bytes = checkForData();
  if (bytes > 0) {
    uint8_t response[3] = {0xAA, 0xBB, 0xCC};  // Send 3 bytes
    sendData(response, 3);
  }
  delay(10);
}
*/