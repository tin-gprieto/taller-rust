#!/bin/bash

echo "Runiing CAM SYSTEM and MONITORING_APP ..."
gnome-terminal -- sh -c "cargo cams_system;bash"
sleep 3

gnome-terminal -- sh -c "cargo monitoring_app;bash"
