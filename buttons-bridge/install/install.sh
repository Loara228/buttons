#!/bin/bash

PROJECT_ROOT=".."
WORKDIR="$(pwd)/../../shell_scripts"
BINARY_PATH="$(pwd)/../target/release/buttons-bridge"
SERVICE_NAME="buttons-bridge.service"

echo "--- Настройка buttons-bridge Service ---"

echo "Компиляция проекта..."
cd "$PROJECT_ROOT" && cargo build --release
if [ $? -ne 0 ]; then
    echo "Ошибка компиляции!"
    exit 1
fi
cd - > /dev/null

read -p "Введите SERIAL_PORT_NAME [/dev/ttyUSB0]: " SERIAL_PORT_NAME
SERIAL_PORT_NAME=${SERIAL_PORT_NAME:-/dev/ttyUSB0}

read -p "Введите SERIAL_PORT_BAUD_RATE [115200]: " SERIAL_PORT_BAUD_RATE
SERIAL_PORT_BAUD_RATE=${SERIAL_PORT_BAUD_RATE:-115200}

read -p "Введите TRACING_LEVEL_FILTER [2]: " TRACING_LEVEL_FILTER
TRACING_LEVEL_FILTER=${TRACING_LEVEL_FILTER:-2}

mkdir -p "$WORKDIR"

mkdir -p "$WORKDIR"
WORKDIR="$(readlink -f "$WORKDIR")"
BINARY_PATH="$(readlink -f "$BINARY_PATH")"

if [ ! -x "$BINARY_PATH" ]; then
  echo "Бинарник не найден или не исполняемый: $BINARY_PATH"
  exit 1
fi

sudo tee /etc/systemd/system/$SERVICE_NAME > /dev/null <<EOF
[Unit]
Description=buttons-bridge
After=network.target

[Service]
Type=simple
User=root
ExecStart=$BINARY_PATH
WorkingDirectory=$WORKDIR
Environment=SERIAL_PORT_NAME=$SERIAL_PORT_NAME
Environment=BAUD_RATE=$SERIAL_PORT_BAUD_RATE
Environment=TRACING_LEVEL_FILTER=$TRACING_LEVEL_FILTER

[Install]
WantedBy=multi-user.target
EOF

echo "Перезапуск systemd и активация службы..."
sudo systemctl daemon-reload
sudo systemctl enable $SERVICE_NAME
sudo systemctl restart $SERVICE_NAME

echo "--- Готово! ---"
sudo systemctl status $SERVICE_NAME --no-pager
