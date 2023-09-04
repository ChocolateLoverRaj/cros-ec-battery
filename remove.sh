sudo systemctl disable --now cros-ec-battery-health-saver
sudo rm /usr/lib/systemd/system/cros-ec-battery-health-saver.service
sudo systemctl daemon-reload
