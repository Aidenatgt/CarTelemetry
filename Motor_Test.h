#ifndef MyLibrary_h
#define MyLibrary_h

#include "Arduino.h"

class Motor_Test {
public:
    Motor_Test(uint8_t pin);
    void setSpeed(uint8_t speed);
private:
    int pin;
};

#endif