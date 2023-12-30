#include "Motor_Test.h"

Motor_Test::Motor_Test(uint8_t pin) {
    pinMode(pin, OUTPUT);
    this->pin = pin;
}

void Motor_Test::setSpeed(uint8_t speed) {
    analogWrite(pin, speed);
}