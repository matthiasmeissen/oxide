// ESP32 I2C Slave - State-Based Model using Wire.h

#include <Wire.h>

// ===== CONFIGURATION =====
#define I2C_ADDRESS 0x08
#define SDA_PIN 21
#define SCL_PIN 22
#define BUTTON_PIN 14

// 1. Define the structure of our state.
//    The __attribute__((packed)) ensures no memory padding is added by the compiler.
struct State {
  bool button1;
  int32_t encoder; // Use fixed-size type to match Rust's i32
};

// 2. Create a union to easily convert the struct to a byte array for sending.
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

  // --- Start of Critical Section ---
  // Disable interrupts to ensure an atomic read of the global state.
  noInterrupts();
  
  // Perform a raw memory copy. This is the correct way to copy a volatile struct.
  memcpy(&local_state_copy, (const void*)&g_state, sizeof(StateUnion));
  
  // Re-enable interrupts immediately after the copy is done.
  interrupts();
  // --- End of Critical Section ---

  // Now we can safely send the consistent, local copy.
  Wire.write(local_state_copy.asBytes, sizeof(local_state_copy.asStruct));
}

void setup() {
  Serial.begin(9600);
  delay(1000);
  Serial.println("\n=== ESP32 I2C State Server ===");

  pinMode(SDA_PIN, INPUT_PULLUP);
  pinMode(SCL_PIN, INPUT_PULLUP);
  pinMode(BUTTON_PIN, INPUT_PULLUP);

  // Initialize the state
  g_state.asStruct.button1 = false;
  g_state.asStruct.encoder = 0; // Initialize future encoder value

  Wire.begin(I2C_ADDRESS);
  Wire.onRequest(requestEvent); // We only need onRequest for this model.
  
  Serial.println("✓ I2C Slave initialized. Broadcasting state on request...\n");
}

void loop() {
  // The logic in the main loop is now incredibly simple.
  // We just update the state struct directly.

  // Read the button. LOW means it's pressed.
  bool is_pressed = (digitalRead(BUTTON_PIN) == LOW);

  // Update the state. This handles press, hold, and release all in one line.
  g_state.asStruct.button1 = is_pressed;
  
  // (In the future, you would update g_state.asStruct.encoder here)

  delay(10); // Small delay to prevent CPU spinning
}