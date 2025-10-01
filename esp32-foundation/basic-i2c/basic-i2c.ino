// ESP32 I2C Slave - Robust "Latching" Version using Wire.h library

#include <Wire.h>

// ===== CONFIGURATION =====
#define I2C_ADDRESS 0x08
#define SDA_PIN 21
#define SCL_PIN 22
#define BUTTON_PIN 14

// This variable holds the message for the Pi.
// It MUST be volatile because it is accessed by an interrupt (the requestEvent)
volatile uint8_t messageForPi = 0x00;

// This function is an ISR (Interrupt Service Routine).
// It's called automatically by the Wire library when the Pi requests data.
// Keep it short and fast!
void requestEvent() {
  Wire.write(messageForPi); // Send the current message back to the master
}

void setup() {
  Serial.begin(9600);
  delay(1000);
  Serial.println("\n=== ESP32 I2C Slave (Wire.h Latching Test) ===");

  pinMode(SDA_PIN, INPUT_PULLUP);
  pinMode(SCL_PIN, INPUT_PULLUP);
  pinMode(BUTTON_PIN, INPUT_PULLUP);
  Serial.print("✓ Button configured on GPIO ");
  Serial.println(BUTTON_PIN);

  // Initialize I2C as a slave
  Wire.begin(I2C_ADDRESS);

  // Register the function to run when the master requests data
  Wire.onRequest(requestEvent);
  
  Serial.println("✓ I2C Slave initialized. Ready for button press...\n");
}

void loop() {
  static int lastButtonState = HIGH;
  
  // Check for a button press
  int currentButtonState = digitalRead(BUTTON_PIN);
  if (lastButtonState == HIGH && currentButtonState == LOW) {
      Serial.println("--- Button Pressed! Latching state to 0x01 ---");
      messageForPi = 0x01; // Set the state and leave it there
      delay(50); // Debounce
  }
  lastButtonState = currentButtonState;

  // The main loop does NOT touch the I2C hardware at all.
  // The Wire library handles everything in the background via interrupts.
  delay(10);
}