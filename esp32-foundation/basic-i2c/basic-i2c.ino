#include <Wire.h>

// Define the I2C address for the ESP32.
// This can be any address from 0x08 to 0x77.
#define I2C_SLAVE_ADDR 0x08  // ESP32's I2C address (you can change this between 0x08-0x77)
#define SDA_PIN 21           // Default SDA pin for ESP32
#define SCL_PIN 22           // Default SCL pin for ESP32

void setup() {
  // Initialize I2C as slave with address 0x08
  Wire.begin(I2C_SLAVE_ADDR, SDA_PIN, SCL_PIN);

  // Register a function to be called when data is received from the master.
  Wire.onReceive(receiveEvent);

  // Initialize Serial Monitor for debugging (optional, but good practice).
  Serial.begin(9600);
  Serial.println("ESP32 I2C Receiver Initialized");
  Serial.print("Listening on Address: 0x");
  Serial.println(I2C_SLAVE_ADDR, HEX);
}

void loop() {
  // The main loop can be empty for this example.
  // All the I2C action happens in the 'receiveEvent' function.
  delay(100);
}

// This function is executed whenever data is received from an I2C master.
// 'howMany' is the number of bytes received.
void receiveEvent(int howMany) {
  Serial.print("Received: ");
  while (Wire.available()) { // loop through all bytes received
    char c = Wire.read();    // receive byte as a character
    Serial.print(c);         // print the character
  }
  Serial.println(); // new line
}