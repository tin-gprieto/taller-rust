#!/bin/bash

echo "Runiing BROKER and MONITORING_APP ..."
gnome-terminal -- sh -c "cargo broker;bash"
sleep 3

gnome-terminal -- sh -c "cargo monitoring_app;bash"
