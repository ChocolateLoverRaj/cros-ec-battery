sudo cp ./cros-ec-battery-health-saver.service /usr/lib/systemd/system
sudo systemctl daemon-reload
sudo systemctl enable --now cros-ec-battery-health-saver
echo "Started \`cros-ec-battery-health-saver\`. Run \`systemctl status cros-ec-battery-health-saver\` for more details."
