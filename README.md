Basic Frame for using mqtt as environment across mulitple windows machines.  

# Usage  
```cmd
  cd %userprofile%/bin  
  Git clone git@github.com:DanEdens/mqttenv.git  
  cd mqttenv
  init.cmd
```
 
todo:  
add linux scripts  
add andriod profiles  
change device name from ghost to variable [Fixed](https://github.com/DanEdens/mqttenv/commit/7d4d165cc676eb14f93bee579ab2b1f1399a9369)
  
fet.cmd - Get, for files stored as msg  
fost.cmd - post, msg from a file  
get.cmd - retrieve value and set as local variable with name final/part/of/topic  
ipr.cmd - Pub current Ip address  
nrost.cmd - remove retained topic  
post.cmd - mosquitto_pub wrapper -t %1 -m "%*(after shift)"  
rost.cmd - Post, but with retain flag  
