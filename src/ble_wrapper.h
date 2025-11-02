#ifndef BLE_WRAPPER_H
#define BLE_WRAPPER_H

int ble_init(void);
int ble_start_advertising(void);
void ble_sleep_ms(int32_t milliseconds);

#endif
