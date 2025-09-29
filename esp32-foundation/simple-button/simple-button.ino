// Simple button reading on ESP32

// Instructions
// This uses and ESP32 V4 Dev Board
// Install the esp32 by Espressif Systems Library
// Use the ESP32 Dev Module to send code

struct AppState {
  bool button1_is_pressed;
  bool button2_is_pressed;
};

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

AppState globalState;

Button button1(14);
Button button2(27);

void printCurrentState() {
  Serial.println("--- State Changed ---");
  Serial.print("Button 1 is pressed: ");
  Serial.println(globalState.button1_is_pressed ? "YES" : "NO");
  Serial.print("Button 2 is pressed: ");
  Serial.println(globalState.button2_is_pressed ? "YES" : "NO");
  Serial.println();
}

void setup() {
  Serial.begin(9600);

  // Initialize each button
  button1.begin();
  button2.begin();

  // Initialize our global state to a known default
  globalState.button1_is_pressed = false;
  globalState.button2_is_pressed = false;

  Serial.println("ESP32 State Management Ready");
  printCurrentState(); // Print the initial state
}

void loop() {
  button1.update();
  button2.update();

  // SECOND: Check if the hardware state has changed compared to our application state
  if (button1.isPressed() != globalState.button1_is_pressed ||
      button2.isPressed() != globalState.button2_is_pressed)
  {
    // If a change is detected, update our global state to match the hardware
    globalState.button1_is_pressed = button1.isPressed();
    globalState.button2_is_pressed = button2.isPressed();

    // THEN: Take action based on the new state
    printCurrentState();
  }
}