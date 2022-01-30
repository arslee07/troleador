import socket

server = "example.com"
port = 6667
channel = "#channel"
username = "troleador"

troleo = """
░░░░░▄▄▄▄▀▀▀▀▀▀▀▀▄▄▄▄▄▄░░░░░░░
░░░░░█░░░░▒▒▒▒▒▒▒▒▒▒▒▒░░▀▀▄░░░░
░░░░█░░░▒▒▒▒▒▒░░░░░░░░▒▒▒░░█░░░
░░░█░░░░░░▄██▀▄▄░░░░░▄▄▄░░░░█░░
░▄▀▒▄▄▄▒░█▀▀▀▀▄▄█░░░██▄▄█░░░░█░
█░▒█▒▄░▀▄▄▄▀░░░░░░░░█░░░▒▒▒▒▒░█
█░▒█░█▀▄▄░░░░░█▀░░░░▀▄░░▄▀▀▀▄▒█
░█░▀▄░█▄░█▀▄▄░▀░▀▀░▄▄▀░░░░█░░█░
░░█░░░▀▄▀█▄▄░█▀▀▀▄▄▄▄▀▀█▀██░█░░
░░░█░░░░██░░▀█▄▄▄█▄▄█▄████░█░░░
░░░░█░░░░▀▀▄░█░░░█░█▀██████░█░░
░░░░░▀▄░░░░░▀▀▄▄▄█▄█▄█▄█▄▀░░█░░
░░░░░░░▀▄▄░▒▒▒▒░░░░░░░░░░▒░░░█░
░░░░░░░░░░▀▀▄▄░▒▒▒▒▒▒▒▒▒▒░░░░█░
░░░░░░░░░░░░░░▀▄▄▄▄▄░░░░░░░░█░░
"""

irc = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
print(f"Connecting to server: {server}:{port}")
irc.connect((server, port))
print(f"Setting username: {username}")
irc.send(str.encode("NICK " + username + "\r\n"))
irc.send(str.encode(f"USER {username} * * :{username} \r\n"))

connected = False
while not connected:
    text = irc.recv(512).decode()

    if "366" in text:
        print("Connected to server")
        connected = True

    if "376" in text:
        print(f"Joining channel: {channel}")
        irc.send(str.encode("JOIN " + channel + "\r\n"))

    if text.startswith("PING"):
        irc.send(str.encode("PONG " + text.split()[1] + "\r\n"))

while True:
    text = irc.recv(1024).decode().strip()

    if text.startswith("PING"):
        irc.send(str.encode("PONG " + text.split()[1] + "\r\n"))

    if "!troleo" in text:
        print("Got command: !troleo")
        for i in troleo.strip().split("\n"):
            irc.send(str.encode(f"PRIVMSG {channel} :" + i + "\r\n"))
