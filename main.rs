use std::io::{prelude::*, BufReader, BufWriter, Error};
use std::net::TcpStream;

const SERVER: &str = "example.com:6667";
const NICKNAME: &str = "troleador";
const CHANNEL: &str = "#channel";

const TROLEO: &str = "\
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
░░░░░░░░░░░░░░▀▄▄▄▄▄░░░░░░░░█░░\
";

struct Irc {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Irc {
    fn new(server: &str) -> Result<Irc, Error> {
        let stream = TcpStream::connect(server)?;
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream.try_clone()?);

        let irc = Irc { reader, writer };

        Ok(irc)
    }

    fn send(&mut self, cmd: String) -> Result<(), Error> {
        self.writer.write(&Irc::format_line(&cmd))?;
        self.writer.flush()?;

        Ok(())
    }

    fn receive(&mut self) -> String {
        let mut text = String::new();
        let _ = self.reader.read_line(&mut text);

        text
    }

    fn format_line(s: &String) -> Vec<u8> {
        format!("{}\r\n", s).as_bytes().to_vec()
    }
}

fn main() -> std::io::Result<()> {
    let mut irc = Irc::new(SERVER)?;
    println!("Connected to server: {}", SERVER);

    irc.send(format!("NICK {}\r\n", NICKNAME))?;
    irc.send(format!("USER {} * * :{}", NICKNAME, NICKNAME))?;

    println!("Set username: {}", NICKNAME);

    let mut connected = false;
    while !connected {
        let text = irc.receive();

        if text.starts_with("PING") {
            let t: Vec<&str> = text.split(":").collect();
            irc.send(format!("PONG :{}", t[1]))?;
        }

        if text.contains("366") {
            println!("Connected to channel");
            connected = true;
        }

        if text.contains("376") {
            println!("Joinining channel: {}", CHANNEL);
            irc.send(format!("JOIN {}", CHANNEL))?;
        }
    }

    loop {
        if false {
            break;
        }

        let text = irc.receive();

        if text.starts_with("PING") {
            let t: Vec<&str> = text.split(":").collect();
            irc.send(format!("PONG :{}", t[1]))?;
        }

        if text.contains("!troleo") {
            println!("Got command: !troleo");

            let lines: Vec<&str> = TROLEO.lines().collect();
            for i in lines {
                irc.send(format!("PRIVMSG {} :{}", CHANNEL, i))?;
            }
        }
    }

    Ok(())
}
