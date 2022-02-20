#!/bin/zsh

echo "please enter the message"
read CMSG
date=`date "+%y%m%d"`
git add .
git commit -m "${date} ${CMSG}"
git push origin main
