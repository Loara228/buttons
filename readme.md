
# Buttons

Простенький проект для автоматизации рутины. При нажатии на физическую кнопку компьютер запускает скрипт.

## Компиляция и запись прошивки на плату

Клонируем репозиторий.

```bash
sudo mkdir -p /opt/buttons
sudo chown $(whoami):$(whoami) /opt/buttons && cd /opt/buttons
git clone https://github.com/Loara228/buttons.git
```

Для полной установки нужно скомпилировать и записать прошивку на плату _(в моем случае Arduino Nano ATMEGA328P)_. Вместо описания понятнее будет просто взглянуть на [код (_main.cpp_)](./arduino/src/main.cpp). В моем случае подключено 3 тактовые кнопки.

Конфигурация платы - [platformio.ini](./arduino/platformio.ini)

## Компиляция и установка службы на компьютер

Тут всё просто. Запускаем [install.sh](./buttons-bridge/install/install.sh) и указываем настройки. Пройдёмся по ним:

 - ```SERIAL_PORT_NAME``` - Порт для общения. Чтобы узнать в какой порт мы подключились можно использовать **dmesg**.

    ```bash
    sudo dmesg | grep tty
    ```

   По умолчанию: **/dev/ttyUSB0**

 - ```SERIAL_PORT_BAUD_RATE``` - Скорость общения между платой и службой. При изменении нужно менять конфигурацию прошивки в [platformio.ini](./arduino/platformio.ini).

    По умолчанию: **115200**

 - ```TRACING_LEVEL_FILTER``` - Уровень логирования службы. Таблица значений ниже

    По умолчанию: **2**

| Level | Value | Description |
|---:|:---:|:---|
| **Trace** | `5` | Designates very low priority, often extremely verbose, information |
| **Debug** | `4` | Designates lower priority information |
| **Info** | `3` | Designates useful information |
| **Warn** | `2` | Designates hazardous situations |
| **Error** | `1` | Designates very serious errors |
| **OFF** | `0` | Designates that trace instrumentation should be completely disabled |