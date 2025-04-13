#!/bin/bash

# Configuración
NOMBRE_SERVICIO="metricas-rust"
RUTA_BINARIO="/opt/data_collection_rust/metricas_rust/data_collection_rust"
USUARIO=$(whoami)
GRUPO=$(id -gn)

cargo build --release
sudo cp /home/gdmartin/Documents/ICOM 2025A/systems/data_collection_rust/target/release/data_collection_rust $RUTA_BINARIO

# Verifica si el binario existe
if [ ! -f "$RUTA_BINARIO" ]; then
  echo "No se encontró el binario en: $RUTA_BINARIO"
  exit 1
fi

# Crear archivo .service
echo "Creando archivo de servicio..."
cat <<EOF | sudo tee /etc/systemd/system/${NOMBRE_SERVICIO}.service > /dev/null
[Unit]
Description=Recolector de métricas de sistema en Rust
Wants=${NOMBRE_SERVICIO}.timer

[Service]
Type=oneshot
ExecStart=${RUTA_BINARIO}
WorkingDirectory=$(dirname ${RUTA_BINARIO})
User=${USUARIO}
Group=${GRUPO}
EOF

# Crear archivo .timer
echo "Creando archivo de timer..."
cat <<EOF | sudo tee /etc/systemd/system/${NOMBRE_SERVICIO}.timer > /dev/null
[Unit]
Description=Ejecuta el recolector de métricas cada 5 minutos

[Timer]
OnBootSec=2min
OnUnitActiveSec=5min
Unit=${NOMBRE_SERVICIO}.service

[Install]
WantedBy=timers.target
EOF

# Recargar systemd y habilitar
echo "Recargando systemd y activando el timer..."
sudo systemctl daemon-reexec
sudo systemctl daemon-reload
sudo systemctl enable ${NOMBRE_SERVICIO}.timer
sudo systemctl start ${NOMBRE_SERVICIO}.timer

# Mostrar estado
echo "Servicio y timer instalados correctamente."
sudo systemctl status ${NOMBRE_SERVICIO}.timer
