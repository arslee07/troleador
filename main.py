import socket

server = "example.com"
port = 6667
channel = "#channel"
nickname = "troleador"

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


def cmd(s: str) -> bytes:
    return str.encode(s + "\r\n")


irc = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
print(f"Connecting to server: {server}:{port}")
irc.connect((server, port))
print(f"Setting nickname: {nickname}")
irc.send(cmd(f"NICK {nickname}"))
irc.send(cmd(f"USER {nickname} * * :{nickname}"))

connected = False
while not connected:
    text = irc.recv(512).decode()

    if "366" in text:
        print("Connected to server")
        connected = True

    if "376" in text:
        print(f"Joining channel: {channel}")
        irc.send(cmd(f"JOIN {channel}"))

    if text.startswith("PING"):
        irc.send(cmd(f"PONG {text.split()[1]}"))

while True:
    text = irc.recv(1024).decode().strip()

    if text.startswith("PING"):
        irc.send(cmd(f"PONG {text.split()[1]}"))

    if "!troleo" in text:
        print("Got command: !troleo")
        for i in troleo.strip().split("\n"):
            irc.send(cmd(f"PRIVMSG {channel} :{i}"))
