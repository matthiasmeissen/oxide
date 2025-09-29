// Simple button reading on ESP32

// Instructions
// This uses and ESP32 V4 Dev Board
// Install the esp32 by Espressif Systems Library
// Use the ESP32 Dev Module to send code

/*
 * Button Class (Corrected Version)
 * A reusable class to handle button input with debouncing and state-change detection.
 */
class Button {
  private:
    byte _pin;                          // The GPIO pin the button is connected to
    int _lastStableState = HIGH;        // The previous STABLE state of the button (debounced)
    int _lastReading = HIGH;            // The last raw reading from the pin
    unsigned long _lastDebounceTime = 0; // The last time a bounce was detected
    unsigned long _debounceDelay = 50;  // The debounce time in milliseconds

  public:
    // Constructor: Initializes the button on a specific pin.
    Button(byte pin) {
      _pin = pin;
    }

    // begin(): Sets up the pin mode. Call this in your main setup() function.
    void begin() {
      pinMode(_pin, INPUT_PULLUP);
    }

    // wasPressed(): Checks if the button was just pressed.
    // Returns true only on the transition from HIGH to LOW.
    bool wasPressed() {
      int currentReading = digitalRead(_pin);
      bool event = false;

      // If the reading has changed, it means there's noise or a real press.
      // Reset the debounce timer.
      if (currentReading != _lastReading) {
        _lastDebounceTime = millis();
      }

      // After the debounce delay has passed, we can be sure the state is stable.
      if ((millis() - _lastDebounceTime) > _debounceDelay) {
        // Check if the stable state has changed from the last time we checked.
        if (currentReading != _lastStableState) {
          _lastStableState = currentReading; // Update the stable state

          // If the new stable state is LOW, it means the button was just pressed.
          if (_lastStableState == LOW) {
            event = true; // This is the press event we want to capture!
          }
        }
      }

      // Update the last raw reading for the next loop.
      _lastReading = currentReading;
      return event;
    }
};

Button button1(14);
Button button2(27);

// Create separate counters for each button
int counter1 = 0;
int counter2 = 0;

void setup() {
  Serial.begin(9600);
  
  button1.begin();
  button2.begin();
  
  Serial.println("ESP32 Button Test");
  Serial.println("Press the button!");
}

void loop() {
  // Read the button (LOW = pressed, HIGH = not pressed)
  if (button1.wasPressed()) {
    counter1++;
    Serial.print("Button 1 pressed! Count: ");
    Serial.println(counter1);
  }

  // Check if button 2 was pressed
  if (button2.wasPressed()) {
    counter2++;
    Serial.print("Button 2 pressed! Count: ");
    Serial.println(counter2);
  }
}