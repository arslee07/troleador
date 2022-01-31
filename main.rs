use std::io::{prelude::*, BufReader, BufWriter};
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

fn cmd(s: &str) -> Vec<u8> {
    format!("{}\r\n", s).as_bytes().to_vec()
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(SERVER)?;
    println!("Connected to server: {}", SERVER);

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream.try_clone()?);

    writer.write(&cmd(&mut format!("NICK {}\r\n", NICKNAME)))?;
    writer.flush()?;
    writer.write(&cmd(&mut format!("USER {} * * :{}", NICKNAME, NICKNAME)))?;
    writer.flush()?;

    println!("Set username: {}", NICKNAME);

    let mut connected = false;
    while !connected {
        let mut text = String::new();
        let _ = reader.read_line(&mut text);

        if text.starts_with("PING") {
            let t: Vec<&str> = text.split(":").collect();
            writer.write(&cmd(&mut format!("PONG :{}", t[1])))?;
            writer.flush()?;
        }

        if text.contains("366") {
            println!("Connected to channel");
            connected = true;
        }

        if text.contains("376") {
            println!("Joinining channel: {}", CHANNEL);
            writer.write(&cmd(&mut format!("JOIN {}", CHANNEL)))?;
            writer.flush()?;
        }
    }

    loop {
        if false {
            break;
        }

        let mut text = String::new();
        let _ = reader.read_line(&mut text);

        if text.starts_with("PING") {
            let t: Vec<&str> = text.split(":").collect();
            writer.write(&cmd(&mut format!("PONG :{}", t[1])))?;
            writer.flush()?;
        }

        if text.contains("!troleo") {
            println!("Got command: !troleo");

            let lines: Vec<&str> = TROLEO.lines().collect();
            for i in lines {
                stream.write(&cmd(&mut format!("PRIVMSG {} :{}", CHANNEL, i)))?;
                writer.flush()?;
            }
        }
    }

    Ok(())
}
