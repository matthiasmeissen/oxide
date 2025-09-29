// Simple button reading on ESP32

// Instructions
// This uses and ESP32 V4 Dev Board
// Install the esp32 by Espressif Systems Library
// Use the ESP32 Dev Module to send code

const int BUTTON1_PIN = 14;
const int BUTTON2_PIN = 27;
const int POT_PIN = 32;

struct AppState {
  bool button1_is_pressed;
  bool button2_is_pressed;
  int potentiometer_value;
};

AppState globalState;

/*
 * Button Class
 * A reusable class to handle button input with debouncing.
 */
class Button {
  private:
    byte _pin;                          // The GPIO pin the button is connected to
    int _lastReading = HIGH;            // The last raw reading from the pin
    int _stableState = HIGH;            // The debounced, stable state of the button
    unsigned long _lastDebounceTime = 0; // The last time a bounce was detected
    unsigned long _debounceDelay = 50;  // The debounce time in milliseconds

  public:
    // Constructor
    Button(byte pin) {
      _pin = pin;
    }

    // begin(): Sets up the pin mode. Call this in your main setup().
    void begin() {
      pinMode(_pin, INPUT_PULLUP);
    }

    // update(): Reads and debounces the button. MUST be called once per loop.
    void update() {
      int currentReading = digitalRead(_pin);

      if (currentReading != _lastReading) {
        _lastDebounceTime = millis();
      }

      if ((millis() - _lastDebounceTime) > _debounceDelay) {
        // The reading has been stable for long enough. Update the stable state.
        _stableState = currentReading;
      }
      
      _lastReading = currentReading;
    }

    // isPressed(): Returns the current debounced state.
    // true if the button is being held down, false otherwise.
    bool isPressed() {
      return (_stableState == LOW);
    }
};

int readPotentiometerSmooth() {
  const int NUM_READINGS = 10; // Number of samples to take
  int total = 0;

  for (int i = 0; i < NUM_READINGS; i++) {
    total += analogRead(POT_PIN);
    delay(1); // Small delay between readings
  }
  return total / NUM_READINGS; // Return the average
}

void printCurrentState() {
  Serial.println("--- State Changed ---");
  Serial.print("Button 1 is pressed: ");
  Serial.println(globalState.button1_is_pressed ? "YES" : "NO");
  Serial.print("Button 2 is pressed: ");
  Serial.println(globalState.button2_is_pressed ? "YES" : "NO");
  Serial.print("Potentiometer Value: ");
  Serial.println(globalState.potentiometer_value); // Print the new value
  Serial.println();
}

Button button1(BUTTON1_PIN);
Button button2(BUTTON2_PIN);

void setup() {
  Serial.begin(9600);

  // Initialize each button
  button1.begin();
  button2.begin();

  // Initialize our global state to a known default
  globalState.button1_is_pressed = false;
  globalState.button2_is_pressed = false;
  globalState.potentiometer_value = 0;

  Serial.println("ESP32 State Management Ready");
  printCurrentState(); // Print the initial state
}

void loop() {
  button1.update();
  button2.update();
  bool b1_isPressed = button1.isPressed();
  bool b2_isPressed = button2.isPressed();
  int pot_value = readPotentiometerSmooth();

  const int POT_CHANGE_THRESHOLD = 40;
  bool pot_has_changed = abs(pot_value - globalState.potentiometer_value) > POT_CHANGE_THRESHOLD;

  // SECOND: Check if the hardware state has changed compared to our application state
  if (b1_isPressed != globalState.button1_is_pressed ||
      b2_isPressed != globalState.button2_is_pressed ||
      pot_has_changed)
  {
    // --- THIRD: If a change is detected, update the global state ---
    globalState.button1_is_pressed = b1_isPressed;
    globalState.button2_is_pressed = b2_isPressed;
    if (pot_has_changed) {
      globalState.potentiometer_value = pot_value;
    }

    // --- FOURTH: Take action based on the new state ---
    printCurrentState();
  }
}