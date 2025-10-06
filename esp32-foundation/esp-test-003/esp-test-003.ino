// ESP32 I2C State Server with Rotary Encoders
// Improved version with thread safety and debouncing
#include <Wire.h>
#include <AiEsp32RotaryEncoder.h>

// ===== I2C CONFIGURATION =====
#define I2C_ADDRESS 0x08
#define SDA_PIN 21
#define SCL_PIN 22

// ===== COMPONENT PIN CONFIGURATION =====
// Standalone buttons
#define BUTTON1_PIN 14
#define BUTTON2_PIN 15  // Add second button when ready

// Encoder 1
#define ENC1_CLK_PIN 32
#define ENC1_DT_PIN 33
#define ENC1_SW_PIN 25

// Encoder 2-4 (uncomment when ready to scale)
// #define ENC2_CLK_PIN 19
// #define ENC2_DT_PIN 23
// #define ENC2_SW_PIN 5

// #define ENC3_CLK_PIN 17
// #define ENC3_DT_PIN 16
// #define ENC3_SW_PIN 18

// #define ENC4_CLK_PIN 26
// #define ENC4_DT_PIN 27
// #define ENC4_SW_PIN 13

// ===== DEBOUNCE CONFIGURATION =====
#define DEBOUNCE_DELAY_MS 50

// ===== STATE DEFINITION =====
#pragma pack(push, 1) // Ensure no padding between bytes
struct State {
  bool button1;       // Byte 0
  bool button2;       // Byte 1
  bool enc1_button;   // Byte 2
  int32_t enc1_value; // Byte 3, 4, 5, 6
  
  // Encoders 2-4 (uncomment when scaling)
  // bool enc2_button;
  // int32_t enc2_value;
  
  // bool enc3_button;
  // int32_t enc3_value;
  
  // bool enc4_button;
  // int32_t enc4_value;
};
#pragma pack(pop)

// Union to easily convert the struct to a byte array for I2C
union StateUnion {
  State asStruct;
  uint8_t asBytes[sizeof(State)];
};

// Global state - protected by critical sections
volatile StateUnion g_state;

// ===== ENCODER INSTANCES =====
AiEsp32RotaryEncoder encoder1 = AiEsp32RotaryEncoder(
  ENC1_DT_PIN, ENC1_CLK_PIN, ENC1_SW_PIN, -1, 1
);

// Uncomment when scaling:
// AiEsp32RotaryEncoder encoder2 = AiEsp32RotaryEncoder(ENC2_DT_PIN, ENC2_CLK_PIN, ENC2_SW_PIN, -1);
// AiEsp32RotaryEncoder encoder3 = AiEsp32RotaryEncoder(ENC3_DT_PIN, ENC3_CLK_PIN, ENC3_SW_PIN, -1);
// AiEsp32RotaryEncoder encoder4 = AiEsp32RotaryEncoder(ENC4_DT_PIN, ENC4_CLK_PIN, ENC4_SW_PIN, -1);

long floor_div(long a, long b) {
  long result = a / b;
  // Correct for negative numbers
  if ((a % b != 0) && ((a < 0) != (b < 0))) {
    result--;
  }
  return result;
}

// ===== DEBOUNCED BUTTON CLASS =====
class DebouncedButton {
private:
  uint8_t pin;
  unsigned long lastChangeTime;
  bool lastReading;
  bool stableState;
  
public:
  DebouncedButton(uint8_t p) : pin(p), lastChangeTime(0), lastReading(false), stableState(false) {}
  
  void begin() {
    pinMode(pin, INPUT_PULLUP);
    lastReading = digitalRead(pin);
    stableState = lastReading;
  }
  
  bool read() {
    bool reading = (digitalRead(pin) == LOW); // LOW = pressed
    
    // If reading changed, reset timer
    if (reading != lastReading) {
      lastChangeTime = millis();
      lastReading = reading;
    }
    
    // If enough time has passed, update stable state
    if ((millis() - lastChangeTime) > DEBOUNCE_DELAY_MS) {
      stableState = reading;
    }
    
    return stableState;
  }
};

// ===== BUTTON INSTANCES =====
DebouncedButton button1(BUTTON1_PIN);
DebouncedButton button2(BUTTON2_PIN);

// ===== I2C INTERRUPT HANDLER =====
void requestEvent() {
  // Create local copy in critical section to avoid torn reads
  StateUnion localStateCopy;
  
  noInterrupts();
  memcpy(&localStateCopy, (const void*)&g_state, sizeof(StateUnion));
  interrupts();
  
  // Send the complete state to I2C master
  Wire.write(localStateCopy.asBytes, sizeof(localStateCopy.asStruct));
}

// ===== ARDUINO SETUP =====
void setup() {
  Serial.begin(9600);
  delay(1000);
  Serial.println("\n=== ESP32 I2C State Server (Improved) ===");
  Serial.printf("State size: %d bytes\n", sizeof(State));
  
  // Configure I2C pins
  pinMode(SDA_PIN, INPUT_PULLUP);
  pinMode(SCL_PIN, INPUT_PULLUP);
  
  // Initialize buttons
  button1.begin();
  button2.begin();
  Serial.println("✓ Buttons initialized");

  // Initialize encoder 1 pins. This is the crucial missing step.
  pinMode(ENC1_CLK_PIN, INPUT_PULLUP);
  pinMode(ENC1_DT_PIN, INPUT_PULLUP);
  pinMode(ENC1_SW_PIN, INPUT_PULLUP); // Be explicit for the button too
  
  // Initialize encoder 1
  encoder1.begin();
  encoder1.setup(
    [] { encoder1.readEncoder_ISR(); },
    [] { /* Button handled separately */ }
  );
  encoder1.disableAcceleration();
  encoder1.setBoundaries(-1000, 1000, true); // Adjust range as needed
  Serial.println("✓ Encoder 1 initialized");
  
  // Uncomment when scaling:
  // encoder2.begin();
  // encoder2.setup([] { encoder2.readEncoder_ISR(); }, [] {});
  // encoder2.setBoundaries(-1000, 1000, true);
  // Serial.println("✓ Encoder 2 initialized");
  
  // encoder3.begin();
  // encoder3.setup([] { encoder3.readEncoder_ISR(); }, [] {});
  // encoder3.setBoundaries(-1000, 1000, true);
  // Serial.println("✓ Encoder 3 initialized");
  
  // encoder4.begin();
  // encoder4.setup([] { encoder4.readEncoder_ISR(); }, [] {});
  // encoder4.setBoundaries(-1000, 1000, true);
  // Serial.println("✓ Encoder 4 initialized");
  
  // Initialize state to known values
  memset((void*)&g_state, 0, sizeof(StateUnion));
  
  // Initialize I2C slave
  Wire.begin(I2C_ADDRESS);
  Wire.onRequest(requestEvent);
  
  Serial.printf("✓ I2C slave ready at address 0x%02X\n", I2C_ADDRESS);
  Serial.println("Ready for requests!\n");
}

// ===== ARDUINO LOOP =====
void loop() {
  // Build state locally first (outside critical section)
  State localState;
  
  // Read all inputs
  localState.button1 = button1.read();
  localState.button2 = button2.read();

  long raw_encoder_steps = encoder1.readEncoder();
  localState.enc1_value = floor_div(raw_encoder_steps, 4);
  localState.enc1_button = encoder1.isEncoderButtonDown();
  
  // Uncomment when scaling:
  // localState.enc2_value = encoder2.readEncoder();
  // localState.enc2_button = encoder2.isEncoderButtonDown();
  
  // localState.enc3_value = encoder3.readEncoder();
  // localState.enc3_button = encoder3.isEncoderButtonDown();
  
  // localState.enc4_value = encoder4.readEncoder();
  // localState.enc4_button = encoder4.isEncoderButtonDown();
  
  // Update global state atomically
  noInterrupts();
  memcpy((void*)&g_state.asStruct, &localState, sizeof(State));
  interrupts();
  
  // Optional: Print state for debugging (comment out in production)
  static unsigned long lastPrint = 0;
  if (millis() - lastPrint > 500) {
    Serial.printf("BTN1:%d BTN2:%d | ENC1: val=%d btn=%d\n",
                  localState.button1,
                  localState.button2,
                  localState.enc1_value,
                  localState.enc1_button);
    lastPrint = millis();
  }
  
  delay(4); // 100Hz update rate
}