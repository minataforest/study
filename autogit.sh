read -p "enter the commit message: " CMSG
git config --global core.autocrlf true
git add .
git commit -m "${CMSG}"
git push origin main
echo "success push"
