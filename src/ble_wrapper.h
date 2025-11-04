#ifndef BLE_WRAPPER_H
#define BLE_WRAPPER_H

#include <zephyr/device.h>

int ble_init(void);
int ble_start_advertising(void);
void sleep_ms(int32_t milliseconds);
int get_light_value(const struct device* dev);

#endif
