#include <Wire.h>

#define I2C_ADDRESS 0x08
#define SDA_PIN 21
#define SCL_PIN 22
#define BUTTON_PIN 14

struct State {
  bool button1;
  int32_t encoder;
};

// Create a union to easily convert the struct to a byte array for sending.
union StateUnion {
  State asStruct;
  uint8_t asBytes[sizeof(State)];
};

// 3. Create a single, global, volatile instance of our state.
//    "volatile" is essential because this is accessed by an interrupt (requestEvent).
volatile StateUnion g_state;

// This ISR is called by the Wire library when the Pi requests data.
void requestEvent() {
  StateUnion local_state_copy;

  // Disable interrupts to ensure an atomic read of the global state.
  noInterrupts();
  
  // Perform a raw memory copy. This is the correct way to copy a volatile struct.
  memcpy(&local_state_copy, (const void*)&g_state, sizeof(StateUnion));
  
  // Re-enable interrupts immediately after the copy is done.
  interrupts();

  Wire.write(local_state_copy.asBytes, sizeof(local_state_copy.asStruct));
}

void setup() {
  Serial.begin(9600);
  delay(1000);
  Serial.println("\n=== ESP32 I2C State Server ===");

  // This is for the I2C Connection
  pinMode(SDA_PIN, INPUT_PULLUP);
  pinMode(SCL_PIN, INPUT_PULLUP);
  pinMode(BUTTON_PIN, INPUT_PULLUP);

  g_state.asStruct.button1 = false;
  g_state.asStruct.encoder = 0;

  Wire.begin(I2C_ADDRESS);
  Wire.onRequest(requestEvent);
  
  Serial.println("✓ I2C Slave initialized. Broadcasting state on request...\n");
}

void loop() {
  // Read the button. LOW means it's pressed.
  bool is_pressed = (digitalRead(BUTTON_PIN) == LOW);

  // Update the state. This handles press, hold, and release all in one line.
  g_state.asStruct.button1 = is_pressed;
  

  delay(10); // Small delay to prevent CPU spinning
}