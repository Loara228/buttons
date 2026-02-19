#!/bin/bash

SERVICE_NAME="buttons-bridge.service"

echo "--- Удаление buttons-bridge Service ---"

if systemctl is-active --quiet $SERVICE_NAME; then
    echo "Остановка службы..."
    sudo systemctl stop $SERVICE_NAME
fi

if systemctl is-enabled --quiet $SERVICE_NAME; then
    echo "Отключение автозагрузки..."
    sudo systemctl disable $SERVICE_NAME
fi

if [ -f "/etc/systemd/system/$SERVICE_NAME" ]; then
    echo "Удаление файла службы..."
    sudo rm "/etc/systemd/system/$SERVICE_NAME"
fi

sudo systemctl daemon-reload
sudo systemctl reset-failed

read -p "Удалить скомпилированный бинарник (target/release)? [y/N]: " clean_bin
if [[ "$clean_bin" =~ ^[Yy]$ ]]; then
    cd .. && cargo clean
    echo "Проект очищен."
fi

echo "--- Служба успешно удалена ---"
