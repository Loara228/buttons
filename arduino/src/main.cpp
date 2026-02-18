#include <Arduino.h>
#include <btn.h>

#define PIN_BUZZER 3

Button btn6 = Button(6); // digital 6
Button btn4 = Button(4); // digital 4
Button btn2 = Button(2); // digital 2

void setup()
{
    Serial.begin(115200);

    pinMode(PIN_BUZZER, OUTPUT);
    tone(PIN_BUZZER, 1000);
    delay(200);
    tone(PIN_BUZZER, 1500);
    delay(200);
    tone(PIN_BUZZER, 1000);
    delay(200);
    noTone(PIN_BUZZER);
}

void send_command(const char *command)
{
    Serial.println(command);
    tone(PIN_BUZZER, 8000);
    delay(100);
    noTone(PIN_BUZZER);
}

void loop()
{
    if (btn2.isPressed())
    {
        send_command("runfile network_restart");
    }
    if (btn4.isPressed())
    {
        send_command("runfile avreg_restart");
    }
    if (btn6.isPressed())
    {
        send_command("runfile test");
    }

    btn2.tick();
    btn4.tick();
    btn6.tick();
    
    delay(25);
}