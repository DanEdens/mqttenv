@mosquitto_pub -q 0 -h %awsip2% -p %awsport% -i %DENA% -t "%1" -r -n
@echo Clearing retained message for topic: %1

