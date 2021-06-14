Basic Frame for using mqtt as environment across mulitple windows machines.  
  
todo:  
add linux scripts  
add andriod profiles  
change device name from ghost to variable  
  
fet.cmd - Get, for files stored as msg  
fost.cmd - post, msg from a file  
get.cmd - retrieve value and set as local variable with name final/part/of/topic  
ipr.cmd - Pub current Ip address  
nrost.cmd - remove retained topic  
post.cmd - mosquitto_pub wrapper -t %1 -m "%*(after shift)"  
rost.cmd - Post, but with retain flag  
