read -p "enter the commit message: " CMSG
git add .
git commit -m "${CMSG}"
git push origin main
echo "success push"