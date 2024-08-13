#!/bin/bash

echo "Running MONITORING_APP ..."
gnome-terminal -- sh -c "cargo monitoring_app;bash"
sleep 3

echo "Running CAMS_SYSTEM ..."
gnome-terminal -- sh -c "cargo cams_system;bash"
sleep 2


