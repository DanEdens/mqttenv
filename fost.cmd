@mosquitto_pub -r -h %awsip% -p %awsport% -i ghost_actual -t "%1" -f "%2" && echo Topic: %1 && echo file: %2
