#!/bin/bash

echo "Running BROKER ... "
gnome-terminal -- sh -c "cargo broker;bash"
sleep 2

echo "Running MONITORING_APP ..."
gnome-terminal -- sh -c "cargo monitoring_app;bash"
sleep 3

echo "Running CAMS_SYSTEM ..."
gnome-terminal -- sh -c "cargo cams_system;bash"
sleep 2

#read  -n 1 -p "PRESIONA UNA TECLA CUANDO MONITORING APP SE HAYA CONECTADO ... "