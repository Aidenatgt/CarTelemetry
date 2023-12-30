#include <Motor_Test.h>

Motor_Test test(10);

void setup() {
}

void loop() {
    test.setSpeed(255);
    delay(5000);
    test.setSpeed(127);
    delay(3000);
    test.setSpeed(0);
    delay(5000);
}
