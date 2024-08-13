#!/bin/bash

echo "Running BROKER ... "
gnome-terminal -- sh -c "cargo broker;bash"
sleep 2

read  -n 1 -p "PRESIONA UNA TECLA CUANDO MONITORING APP SE HAYA CONECTADO ... "
echo " "

for DRONE in 1 2 3 4 5 6 7 8 9 10 11 12 13
do
    echo "Running DRONE: <$DRONE> ..."
    gnome-terminal -- sh -c "cargo drone$DRONE;bash"
	sleep 1
done