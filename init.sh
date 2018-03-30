echo "Please enter following info about your new project!"

if hash python3 2>/dev/null; then
    python3 init.py
else
    echo "Looks like you don't have python3 installed ._."
    echo "Just edit the Cargo.toml manually!"
fi

sh ./remove-init.sh
echo "Removed all init scripts!"
echo "Happy Programming!"
