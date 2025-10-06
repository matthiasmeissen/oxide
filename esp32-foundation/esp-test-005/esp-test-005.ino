// ESP32 I2C State Server with Rotary Encoders
#include <Wire.h>
#include <AiEsp32RotaryEncoder.h>

// ===== I2C CONFIGURATION =====
#define I2C_ADDRESS 0x08
#define SDA_PIN 21
#define SCL_PIN 22

// ===== COMPONENT PIN CONFIGURATION =====
#define BUTTON1_PIN 25    // Input Pullup
#define BUTTON2_PIN 26    // Input Pullup
#define BUTTON3_PIN 27    // Input Pullup
#define BUTTON4_PIN 14    // Input Pullup
#define POT1_PIN 34       // ADC1 - Input and Output
#define POT2_PIN 35       // ADC1 - Input and Output
#define POT3_PIN 32       // ADC1 - Input only
#define POT4_PIN 33       // ADC1 - Input only
#define ENC1_SW_PIN 13    // Input Pullup
#define ENC1_CLK_PIN 19   // Input Pullup
#define ENC1_DT_PIN 18    // Input Pullup

// ===== DEBOUNCE CONFIGURATION =====
#define DEBOUNCE_DELAY_MS 50
#define POT_SMOOTHING_FACTOR 0.1 // Low value = very smooth. Range: 0.0 to 1.0
#define POT_HYSTERESIS_THRESHOLD 8 // The value must change by at least 8 (out of 4095) to be reported

// ===== STATE DEFINITION =====
#pragma pack(push, 1) // Ensure no padding between bytes
struct State {
  bool button1;         // Byte 0
  bool button2;         // Byte 1
  bool button3;         // Byte 2
  bool button4;         // Byte 3
  uint16_t pot1_value;  // Byte 4, 5
  uint16_t pot2_value;  // Byte 6, 7
  uint16_t pot3_value;  // Byte 8, 9
  uint16_t pot4_value;  // Byte 10, 11
  bool enc1_button;     // Byte 12
  int32_t enc1_value;   // Byte 13, 14, 15, 16
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
AiEsp32RotaryEncoder encoder1 = AiEsp32RotaryEncoder(ENC1_DT_PIN, ENC1_CLK_PIN, ENC1_SW_PIN, -1, 1);

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
DebouncedButton button3(BUTTON3_PIN);
DebouncedButton button4(BUTTON4_PIN);

// ===== SMOOTHING CLASS =====
class SmoothedAnalogInput {
private:
  uint8_t pin;
  float alpha;
  float smoothedValue;

public:
  SmoothedAnalogInput(uint8_t p, float a) : pin(p), alpha(a), smoothedValue(0.0) {}

  void begin() {
    // Initialize by taking a first reading
    smoothedValue = analogRead(pin);
  }

  uint16_t read() {
    // Perform one step of the Exponential Moving Average
    float rawValue = analogRead(pin);
    smoothedValue = (rawValue * alpha) + (smoothedValue * (1.0 - alpha));
    return (uint16_t)(smoothedValue + 0.5); // Return a rounded integer
  }
};

// SmoothedAnalogInput pot1(POT1_PIN, POT1_SMOOTHING_FACTOR);

// ===== ANALOG INPUT CLASS =====
class AnalogInput {
private:
  uint8_t pin;
  float alpha; // For smoothing
  uint16_t hysteresisThreshold; // For stability
  float smoothedValue;
  uint16_t lastReportedValue; // The last value we actually sent

public:
  AnalogInput(uint8_t p, float a, uint16_t h) :
    pin(p), alpha(a), hysteresisThreshold(h), smoothedValue(0.0), lastReportedValue(0) {}

  void begin() {
    smoothedValue = analogRead(pin);
    lastReportedValue = smoothedValue;
  }

  // This function now returns a stable, 12-bit value (0-4095)
  uint16_t read() {
    // 1. Smoothing (EMA Filter)
    float rawValue = analogRead(pin);
    smoothedValue = (rawValue * alpha) + (smoothedValue * (1.0 - alpha));

    // 2. Hysteresis Check
    // Only update the reported value if the new smoothed value has
    // moved past the threshold from the last reported value.
    if (abs(smoothedValue - lastReportedValue) > hysteresisThreshold) {
      lastReportedValue = (uint16_t)(smoothedValue + 0.5);
    }
    
    return lastReportedValue;
  }
};

AnalogInput pot1(POT1_PIN, POT_SMOOTHING_FACTOR, POT_HYSTERESIS_THRESHOLD);
AnalogInput pot2(POT2_PIN, POT_SMOOTHING_FACTOR, POT_HYSTERESIS_THRESHOLD);
AnalogInput pot3(POT3_PIN, POT_SMOOTHING_FACTOR, POT_HYSTERESIS_THRESHOLD);
AnalogInput pot4(POT4_PIN, POT_SMOOTHING_FACTOR, POT_HYSTERESIS_THRESHOLD);

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
  button3.begin();
  button4.begin();
  Serial.println("✓ Buttons initialized");

  // Initialize Potentiometer
  pot1.begin();
  pot2.begin();
  pot3.begin();
  pot4.begin();
  Serial.println("✓ Potentiometer initialized");

  // Initialize encoder 1 pins
  pinMode(ENC1_CLK_PIN, INPUT_PULLUP);
  pinMode(ENC1_DT_PIN, INPUT_PULLUP);
  pinMode(ENC1_SW_PIN, INPUT_PULLUP);
  
  // Initialize encoder 1
  encoder1.begin();
  encoder1.setup(
    [] { encoder1.readEncoder_ISR(); },
    [] { /* Button handled separately */ }
  );
  encoder1.disableAcceleration();
  encoder1.setBoundaries(-256, 256, true);
  Serial.println("✓ Encoder 1 initialized");
  
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
  
  // Read Buttons
  localState.button1 = button1.read();
  localState.button2 = button2.read();
  localState.button3 = button3.read();
  localState.button4 = button4.read();

  // Read Pots
  uint16_t stablePotValue1 = pot1.read();
  localState.pot1_value = stablePotValue1 / 4;

  uint16_t stablePotValue2 = pot2.read();
  localState.pot2_value = stablePotValue2 / 4;

  uint16_t stablePotValue3 = pot3.read();
  localState.pot3_value = stablePotValue3 / 4;

  uint16_t stablePotValue4 = pot4.read();
  localState.pot4_value = stablePotValue4 / 4;

  // Read Encoder
  long raw_encoder_steps = encoder1.readEncoder();
  localState.enc1_value = floor_div(raw_encoder_steps, 4);
  localState.enc1_button = encoder1.isEncoderButtonDown();

  
  // Update global state atomically
  noInterrupts();
  memcpy((void*)&g_state.asStruct, &localState, sizeof(State));
  interrupts();
  
  // Optional: Print state for debugging (comment out in production)
  static unsigned long lastPrint = 0;
  if (millis() - lastPrint > 500) {
    Serial.println("--------");
    Serial.printf("BTN1:%d BTN2:%d BTN3:%d BTN4:%d \n",
                  localState.button1,
                  localState.button2,
                  localState.button3,
                  localState.button4);
    Serial.printf("POT1: val=%d POT2: val=%d POT3: val=%d POT4: val=%d \n",
                  localState.pot1_value,
                  localState.pot2_value,
                  localState.pot3_value,
                  localState.pot4_value);
    Serial.printf("ENC1: val=%d btn=%d \n",
                  localState.enc1_value,
                  localState.enc1_button);
    lastPrint = millis();
  }
  
  delay(4); // 100Hz update rate
}