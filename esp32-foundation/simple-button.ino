// Simple button reading on ESP32

#define BUTTON_PIN 2  // GPIO2

void setup() {
  Serial.begin(9600);
  
  // Set up the button pin with internal pull-up resistor
  pinMode(BUTTON_PIN, INPUT_PULLUP);
  
  Serial.println("ESP32 Button Test");
  Serial.println("Press the button!");
}

void loop() {
  // Read the button (LOW = pressed, HIGH = not pressed)
  bool button_pressed = digitalRead(BUTTON_PIN) == LOW;
  
  if (button_pressed) {
    Serial.println("Button PRESSED!");
    delay(200);  // Simple debouncing
  }
  
  delay(50);  // Small delay to prevent overwhelming serial output
}