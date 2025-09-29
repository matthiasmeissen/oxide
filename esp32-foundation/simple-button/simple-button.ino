// Simple button reading on ESP32

// Instructions
// This uses and ESP32 V4 Dev Board
// Install the esp32 by Espressif Systems Library
// Use the ESP32 Dev Module to send code

// All pins are on the left side: 3V3 (Pin number 1) -> GND (Pin number 19)
const int BUTTON1_PIN = 14;   // Left side pin number 12
const int BUTTON2_PIN = 13;   // Left side pin number 15
const int POT_PIN = 32;
const int ENCODER_A_PIN = 25; // CLK
const int ENCODER_B_PIN = 26; // DT
const int ENCODER_SW_PIN = 27; // Encoder's push button switch

struct AppState {
  bool button1_is_pressed;
  bool button2_is_pressed;
  bool encoder_button_is_pressed;
  int potentiometer_value;
  long encoder_count;
};

AppState globalState;


class Button {
  private:
    byte _pin;
    int _lastReading = HIGH;
    int _stableState = HIGH;
    unsigned long _lastDebounceTime = 0;
    unsigned long _debounceDelay = 50;

  public:
    Button(byte pin) { _pin = pin; }
    void begin() { pinMode(_pin, INPUT_PULLUP); }

    // update() is the heart of the class. It reads the pin and filters out
    // the "bouncing" noise from the mechanical switch. It MUST be called every loop.
    void update() {
      int currentReading = digitalRead(_pin);
      if (currentReading != _lastReading) {
        _lastDebounceTime = millis();
      }
      if ((millis() - _lastDebounceTime) > _debounceDelay) {
        _stableState = currentReading;
      }
      _lastReading = currentReading;
    }

    // isPressed() simply returns the clean, debounced state.
    bool isPressed() { return (_stableState == LOW); }
};


// Takes multiple ADC readings and averages them to get a stable value filtering out electrical noise.
int readPotentiometerSmooth() {
  const int NUM_READINGS = 10; int total = 0;
  for (int i = 0; i < NUM_READINGS; i++) {
    total += analogRead(POT_PIN);
    delay(1);
  }
  return total / NUM_READINGS;
}

// Performs mathematically correct floor division for both positive and negative numbers.
long floor_div(long a, long b) {
  long result = a / b;
  if ((a % b != 0) && ((a < 0) != (b < 0))) {
    result--;
  }
  return result;
}

// Print the contents of globalState
void printCurrentState() {
  Serial.println("--- State Changed ---");
  Serial.print("Button 1 is pressed: "); Serial.println(globalState.button1_is_pressed ? "YES" : "NO");
  Serial.print("Button 2 is pressed: "); Serial.println(globalState.button2_is_pressed ? "YES" : "NO");
  Serial.print("Encoder Button is pressed: "); Serial.println(globalState.encoder_button_is_pressed ? "YES" : "NO");
  Serial.print("Potentiometer Value: "); Serial.println(globalState.potentiometer_value);
  Serial.print("Encoder Count: "); Serial.println(globalState.encoder_count);
  Serial.println();
}

// Create an instance of our Button class for each physical button.
Button button1(BUTTON1_PIN);
Button button2(BUTTON2_PIN);
Button encoderButton(ENCODER_SW_PIN);

// --- ISR Global Variables ---
// 'volatile' is a special keyword. It tells the compiler that this variable can
// change unexpectedly at any time, so it must always read the value directly
// from memory. This is ESSENTIAL for variables used in an ISR.
volatile long encoder_steps = 0; // Tracks the high-resolution (X4) encoder count.
volatile int last_encoded = 0;  // Remembers the last state for direction detection.

// --- The Interrupt Service Routine (ISR) ---
// The ESP32 hardware will automatically pause the main
// loop and run this code INSTANTLY when a change on the encoder pins is detected.
// It must be extremely fast. No delays, no Serial prints.
void IRAM_ATTR readEncoderISR() {
  int msb = digitalRead(ENCODER_A_PIN); int lsb = digitalRead(ENCODER_B_PIN);
  int encoded = (msb << 1) | lsb;
  int sum = (last_encoded << 2) | encoded;
  if(sum == 0b1101 || sum == 0b0100 || sum == 0b0010 || sum == 0b1011) encoder_steps++;
  if(sum == 0b1110 || sum == 0b0111 || sum == 0b0001 || sum == 0b1000) encoder_steps--;
  last_encoded = encoded;
}

void setup() {
  Serial.begin(9600);

  // Initialize all hardware objects and pin modes.
  button1.begin();
  button2.begin();
  encoderButton.begin();
  pinMode(ENCODER_A_PIN, INPUT_PULLUP);
  pinMode(ENCODER_B_PIN, INPUT_PULLUP);

  // Tell ESP32 to run 'readEncoderISR' function whenever the state
  // of ENCODER_A_PIN or ENCODER_B_PIN changes.
  attachInterrupt(digitalPinToInterrupt(ENCODER_A_PIN), readEncoderISR, CHANGE);
  attachInterrupt(digitalPinToInterrupt(ENCODER_B_PIN), readEncoderISR, CHANGE);

  // Set our global state to a known, default starting condition.
  globalState = {false, false, false, 0, 0};

  Serial.println("ESP32 Full State Demo Ready");
  printCurrentState();
}

void loop() {
  // Update all hardware that is not interrupt-driven.
  button1.update();
  button2.update();
  encoderButton.update();
  
  bool b1_isPressed = button1.isPressed();
  bool b2_isPressed = button2.isPressed();
  bool encBtn_isPressed = encoderButton.isPressed();
  int pot_value = readPotentiometerSmooth();

  // Safely copy the volatile variable from the ISR into a local variable.
  noInterrupts();
  long current_raw_steps = encoder_steps;
  interrupts();

  // Convert the high-resolution raw data into the final "click" count
  long current_integer_count = floor_div(current_raw_steps, 4);
  
  // Check if current state is different from last recorded globalState
  const int POT_CHANGE_THRESHOLD = 40;
  bool pot_has_changed = abs(pot_value - globalState.potentiometer_value) > POT_CHANGE_THRESHOLD;

  if (b1_isPressed != globalState.button1_is_pressed ||
      b2_isPressed != globalState.button2_is_pressed ||
      encBtn_isPressed != globalState.encoder_button_is_pressed ||
      current_integer_count != globalState.encoder_count ||
      pot_has_changed)
  {
    // If anything changed, we update our globalState to match the new reality.
    globalState.button1_is_pressed = b1_isPressed;
    globalState.button2_is_pressed = b2_isPressed;
    globalState.encoder_button_is_pressed = encBtn_isPressed;
    globalState.encoder_count = current_integer_count;
    if (pot_has_changed) {
      globalState.potentiometer_value = pot_value;
    }

    // Now that the state is updated, we take action (e.g., print the new state).
    printCurrentState();
  }
}
