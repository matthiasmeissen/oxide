// This is intended to be used without the Raspberry PI

// ESP32 I2C State Server with Rotary Encoders AND Display Test
#include <Wire.h>
#include <AiEsp32RotaryEncoder.h>
// 1. INCLUDE THE DISPLAY LIBRARY
#include <U8g2lib.h> 

// ===== I2C CONFIGURATION =====
#define I2C_ADDRESS 0x08
#define SDA_PIN 21
#define SCL_PIN 22

// 2. CONFIGURE DISPLAY
// Using Hardware I2C. The ESP32 becomes the Master.
// Rotation R0, No Reset Pin, uses your defined SCL/SDA
U8G2_SH1106_128X64_NONAME_F_HW_I2C u8g2(U8G2_R0, /* reset=*/ U8X8_PIN_NONE, /* clock=*/ SCL_PIN, /* data=*/ SDA_PIN);

// ===== COMPONENT PIN CONFIGURATION =====
#define BUTTON1_PIN 25    
#define BUTTON2_PIN 26    
#define BUTTON3_PIN 27    
#define BUTTON4_PIN 14    
#define POT1_PIN 34       
#define POT2_PIN 35       
#define POT3_PIN 32       
#define POT4_PIN 33       
#define ENC1_SW_PIN 13    
#define ENC1_CLK_PIN 18   
#define ENC1_DT_PIN 19    

// ===== DEBOUNCE CONFIGURATION =====
#define DEBOUNCE_DELAY_MS 50
#define POT_SMOOTHING_FACTOR 0.25
#define POT_HYSTERESIS_THRESHOLD 8 

// ===== STATE DEFINITION =====
#pragma pack(push, 1) 
struct State {
  bool button1;         
  bool button2;         
  bool button3;         
  bool button4;         
  uint16_t pot1_value;  
  uint16_t pot2_value;  
  uint16_t pot3_value;  
  uint16_t pot4_value;  
  bool enc1_button;     
  int32_t enc1_value;   
};
#pragma pack(pop)

union StateUnion {
  State asStruct;
  uint8_t asBytes[sizeof(State)];
};

volatile StateUnion g_state;

// ===== ENCODER INSTANCES =====
AiEsp32RotaryEncoder encoder1 = AiEsp32RotaryEncoder(ENC1_DT_PIN, ENC1_CLK_PIN, ENC1_SW_PIN, -1, 1);

long floor_div(long a, long b) {
  long result = a / b;
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
    bool reading = (digitalRead(pin) == LOW); 
    if (reading != lastReading) {
      lastChangeTime = millis();
      lastReading = reading;
    }
    if ((millis() - lastChangeTime) > DEBOUNCE_DELAY_MS) {
      stableState = reading;
    }
    return stableState;
  }
};

DebouncedButton button1(BUTTON1_PIN);
DebouncedButton button2(BUTTON2_PIN);
DebouncedButton button3(BUTTON3_PIN);
DebouncedButton button4(BUTTON4_PIN);

// ===== ANALOG INPUT CLASS =====
class AnalogInput {
private:
  uint8_t pin;
  float alpha; 
  uint16_t hysteresisThreshold; 
  float smoothedValue;
  uint16_t lastReportedValue; 

public:
  AnalogInput(uint8_t p, float a, uint16_t h) :
    pin(p), alpha(a), hysteresisThreshold(h), smoothedValue(0.0), lastReportedValue(0) {}

  void begin() {
    smoothedValue = analogRead(pin);
    lastReportedValue = smoothedValue;
  }

  uint16_t read() {
    float rawValue = analogRead(pin);
    smoothedValue = (rawValue * alpha) + (smoothedValue * (1.0 - alpha));

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
  StateUnion localStateCopy;
  noInterrupts();
  memcpy(&localStateCopy, (const void*)&g_state, sizeof(StateUnion));
  interrupts();
  Wire.write(localStateCopy.asBytes, sizeof(localStateCopy.asStruct));
}

// ===== ARDUINO SETUP =====
void setup() {
  Serial.begin(9600);
  delay(1000);
  Serial.println("\n=== ESP32 Display Test Mode ===");

  // 3. INITIALIZE DISPLAY
  // NOTE: This initializes Wire internally as MASTER.
  // We cannot act as a Slave to the Pi and drive the display at the same time easily.
  u8g2.begin();
  u8g2.clearBuffer();
  u8g2.setFont(u8g2_font_6x10_tf);
  u8g2.drawStr(10, 30, "Booting...");
  u8g2.sendBuffer();
  
  // Configure I2C pins (U8g2 handles this, but ensuring pullups is good)
  // pinMode(SDA_PIN, INPUT_PULLUP);
  // pinMode(SCL_PIN, INPUT_PULLUP);
  
  button1.begin();
  button2.begin();
  button3.begin();
  button4.begin();

  pot1.begin();
  pot2.begin();
  pot3.begin();
  pot4.begin();

  pinMode(ENC1_CLK_PIN, INPUT_PULLUP);
  pinMode(ENC1_DT_PIN, INPUT_PULLUP);
  pinMode(ENC1_SW_PIN, INPUT_PULLUP);
  
  encoder1.begin();
  encoder1.setup(
    [] { encoder1.readEncoder_ISR(); },
    [] { }
  );
  encoder1.disableAcceleration();
  encoder1.setBoundaries(-256, 256, true);
  
  memset((void*)&g_state, 0, sizeof(StateUnion));
  
  // 4. DISABLE SLAVE MODE FOR TESTING
  // Since the PI is not connected, we comment this out.
  // u8g2.begin() has already taken control of the I2C bus.
  /*
  Wire.begin(I2C_ADDRESS);
  Wire.onRequest(requestEvent);
  Serial.printf("✓ I2C slave ready at address 0x%02X\n", I2C_ADDRESS);
  */
  
  Serial.println("Display initialized. Running test loop.");
}

// ===== ARDUINO LOOP =====
void loop() {
  State localState;
  
  // ==========================================
  // 1. HIGH SPEED SECTION (Executes as fast as possible)
  // ==========================================
  
  // Read Buttons
  localState.button1 = button1.read();
  localState.button2 = button2.read();
  localState.button3 = button3.read();
  localState.button4 = button4.read();

  // Read Pots 
  // We need to call .read() very frequently for the smoothing math to work correctly
  // without lagging.
  uint16_t r1 = pot1.read();
  uint16_t r2 = pot2.read();
  uint16_t r3 = pot3.read();
  uint16_t r4 = pot4.read();

  localState.pot1_value = 1023 - (r1 / 4);
  localState.pot2_value = 1023 - (r2 / 4);
  localState.pot3_value = 1023 - (r3 / 4);
  localState.pot4_value = 1023 - (r4 / 4);

  // Read Encoder
  long raw_encoder_steps = encoder1.readEncoder();
  localState.enc1_value = floor_div(raw_encoder_steps, 4);
  localState.enc1_button = encoder1.isEncoderButtonDown();

  // Update global state safely
  noInterrupts();
  memcpy((void*)&g_state.asStruct, &localState, sizeof(State));
  interrupts();

  // ==========================================
  // 2. LOW SPEED SECTION (Display & Debug)
  // ==========================================
  
  // Only update display every 100ms (10 FPS)
  // This prevents the slow I2C bus from blocking the pot reading logic
  static unsigned long lastDisplayTime = 0;
  
  if (millis() - lastDisplayTime > 100) { 
    lastDisplayTime = millis();

    // DRAW TO DISPLAY
    u8g2.clearBuffer();
    u8g2.setFont(u8g2_font_6x10_tf);

    // Draw Buttons
    u8g2.drawStr(0, 10, "B:");
    u8g2.drawFrame(15, 2, 8, 8); if(localState.button1) u8g2.drawBox(17, 4, 4, 4);
    u8g2.drawFrame(25, 2, 8, 8); if(localState.button2) u8g2.drawBox(27, 4, 4, 4);
    u8g2.drawFrame(35, 2, 8, 8); if(localState.button3) u8g2.drawBox(37, 4, 4, 4);
    u8g2.drawFrame(45, 2, 8, 8); if(localState.button4) u8g2.drawBox(47, 4, 4, 4);

    // Draw Pots
    u8g2.setCursor(0, 25); u8g2.print("P1:"); u8g2.print(localState.pot1_value);
    u8g2.setCursor(64, 25); u8g2.print("P2:"); u8g2.print(localState.pot2_value);
    u8g2.setCursor(0, 35); u8g2.print("P3:"); u8g2.print(localState.pot3_value);
    u8g2.setCursor(64, 35); u8g2.print("P4:"); u8g2.print(localState.pot4_value);

    // Draw Encoder
    u8g2.setCursor(0, 55); 
    u8g2.print("ENC: "); u8g2.print(localState.enc1_value);
    if (localState.enc1_button) u8g2.print(" [X]");

    u8g2.sendBuffer(); // This takes ~50ms, blocking execution
  }
}