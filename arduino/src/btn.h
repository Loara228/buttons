#ifndef ESPS_BUTTON
#define ESPS_BUTTON

#include <Arduino.h>

#define ESPS_DEBOUNCE_DELAY 50

class Button
{
public:
    enum Flag : uint8_t
    {
        FLAG_NONE = 0,
        FLAG_PRESSED = 1 << 0,
        FLAG_DOWN = 1 << 1,
        FLAG_RELEASED = 1 << 2
    };

    Button(uint8_t u8_pin)
    {
        pin = u8_pin;
        pinMode(pin, INPUT_PULLUP);
        lastState = lastStableReading = digitalRead(pin);
        flags = FLAG_NONE;
        lastDebounceTime = millis();
    }

    void tick()
    {
        uint8_t reading = digitalRead(pin);
        unsigned long now = millis();

        flags &= ~(FLAG_PRESSED | FLAG_RELEASED);

        if (reading != lastState)
        {
            lastDebounceTime = now;
        }

        if ((now - lastDebounceTime) > ESPS_DEBOUNCE_DELAY)
        {
            if (reading != lastStableReading)
            {
                lastStableReading = reading;

                if (lastStableReading == LOW)
                {
                    if (!(flags & FLAG_DOWN))
                    {
                        flags |= FLAG_PRESSED;
                        flags |= FLAG_DOWN;
                    }
                }
                else
                {
                    if (flags & FLAG_DOWN)
                    {
                        flags |= FLAG_RELEASED;
                        flags &= ~FLAG_DOWN;
                    }
                }
            }
        }

        lastState = reading;
    }

    bool isPressed() const { return flags & FLAG_PRESSED; }
    bool isDown() const { return flags & FLAG_DOWN; }
    bool isReleased() const { return flags & FLAG_RELEASED; }

private:
    uint8_t pin;
    uint8_t flags;

    uint8_t lastState = HIGH;
    uint8_t lastStableReading = HIGH;
    unsigned long lastDebounceTime = 0;
};

#endif