#!/bin/bash

for DRONE in 1 2 3 4 5 6 7 8 9 10 11 12 13
do
    echo "Running DRONE: <$DRONE> ..."
    gnome-terminal -- sh -c "cargo drone$DRONE;bash"
	sleep 1
done


