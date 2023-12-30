#include "Motor_Test.h"

extern "C" {
    typedef Motor_Test Motor_TestHandle;

    Motor_TestHandle create(uint8_t pin) {
        return Motor_Test(pin);
    }

    void setSpeed(Motor_TestHandle* handle, uint8_t speed) {
        handle->setSpeed(speed);
    }
}