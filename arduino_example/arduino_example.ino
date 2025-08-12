/*
 * Simple Arduino Example for GdSerial
 * 
 * Basic serial communication demo for the GdSerial library.
 * Baud Rate: 9600
 */

String command = "";
int counter = 0;

void setup() {
  Serial.begin(9600);
  Serial.println("Arduino ready");
}

void loop() {
  // Read commands
  if (Serial.available()) {
    char c = Serial.read();
    if (c == '\n') {
      processCommand();
      command = "";
    } else if (c != '\r') {
      command += c;
    }
  }
  
  delay(10);
}

void processCommand() {
  command.trim();
  command.toLowerCase();
  
  if (command == "ping") {
    Serial.println("pong");
    
  } else if (command == "hello") {
    Serial.println("Hello from Arduino!");
    
  } else if (command.startsWith("echo ")) {
    Serial.println(command.substring(5));
    
  } else if (command == "count") {
    Serial.println(counter++);
    
  } else if (command == "time") {
    Serial.println(millis());
    
  } else if (command == "help") {
    Serial.println("Commands: ping, hello, echo <text>, count, time, help");
    
  } else {
    Serial.println("Unknown command");
  }
}