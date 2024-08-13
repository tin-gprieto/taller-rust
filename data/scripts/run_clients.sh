#!/bin/bash

echo "Running CAMS SYSTEM ... "
gnome-terminal -- sh -c "cargo cams_system;bash"
sleep 2

for DRONE in 1 2 3 4 5 6 7
do
    echo "Running DRONE: <$DRONE> ..."
    gnome-terminal -- sh -c "cargo drone$DRONE;bash"
	sleep 1
done
