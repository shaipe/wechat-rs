#!/bin/bash
echo "查看端口进程"
lsof -i:999
echo -n "请输入进程id"
read input_msg

kill input_msg
chmod 755 wechat
echo "启动应用"
nohup ./wechat &
