import subprocess

# ask for the name of the new project
name = input("project name: ")

# get info about author
author = subprocess.run(["git", "config", "user.name"], stdout=subprocess.PIPE)
author = author.stdout.decode("utf-8").strip("\n")

print("author: " +  author)

email = subprocess.run(["git", "config", "user.email"], stdout=subprocess.PIPE)
email = email.stdout.decode("utf-8").strip("\n")

print("email: " + email)

# write metadata to Cargo.toml
lines = open("Cargo.toml").read().splitlines()

lines[1] = 'name = "{0}"'.format(name)
lines[2] = 'version = "0.1.0"'
lines[3] = 'authors = ["{0} <{1}>"]'.format(author, email)

open("Cargo.toml","w").write("\n".join(lines))
