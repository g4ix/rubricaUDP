use std::net::UdpSocket;
//use std::env;
use std::str;
use std::io::{self, BufRead};


// creazione della struct persona
struct Persona {
    nome: String,
    cognome: String,
}

// splitto la linea di comando per ottenere comando, nome, cognome e hostname
fn split_line(input: &str) -> Vec<String> {
    let mut result = Vec::new();

    if let Some(name_index) = input.find("--name") {
        if let Some(surname_index) = input.find("--surname") {
            if let Some(server_index) = input.find("--server") {
                let command = input[..name_index].trim();
                let name = input[name_index + 7..surname_index].trim();
                let surname = input[surname_index + 9..server_index].trim();
                let hostname = input[server_index + 9..].trim();

                result.push(command.to_string());
                result.push(name.to_string());
                result.push(surname.to_string());
                result.push(hostname.to_string());

                return result;
            }
        }
    }

    result
}

fn split_line2(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let words: Vec<&str> = input.split("--").collect();

    let command = words[0].to_string();
    result.push(command);

    for pair in words {
        if let Some(value) = extract_value(pair) {
            result.push(value);
        }
    }
    result.retain(|value| !value.is_empty());
    result
}

fn extract_value(pair: &str) -> Option<String> {
    let parts: Vec<&str> = pair.splitn(2, ' ').collect();
    if parts.len() == 2 {
        Some(parts[1].trim().to_string())
    } else {
        None
    }
}

// inserimento di una persona
fn client_insert(hostname: &str, persona: Persona, socket: &UdpSocket) -> std::io::Result<()> {

    let mut name = persona.nome;
    let surname = persona.cognome;
    name.push('+');
    name.push_str(&surname);

     // invio la stringa digitata da linea di comando
     socket.send_to(name.as_bytes(), hostname.to_string() + &":2000").expect("Error on send");
    
     Ok(())
}

// visualizzazione della lista delle persone 
// come si fa a ricevere la lista delle persone dal server?
fn client_list(hostname: &str, socket: &UdpSocket) -> std::io::Result<()> {
    
        // invio il comando list al server
        socket.send_to("list".as_bytes(), hostname.to_string() + &":2000").expect("Error on send");
        
        Ok(())
}


fn main() -> std::io::Result<()> {
    
    // creo la socket
    let socket = UdpSocket::bind("127.0.0.1:3401")?;  // for UDP4/6

    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        let line = line.unwrap();
        //println!("Hai digitato: {}", line);
        if &line == "BYE" {
            break;
        }

        let args = split_line2(&line);
       
        let command = &args[0];

        if command=="insert " {
            let name = args[1].to_string();
            let surname = args[2].to_string();
            let hostname = args[3].to_string();


            let persona = Persona {
                nome: name.to_string(),
                cognome: surname.to_string(),
            };

            if let Err(err) = client_insert(&hostname, persona, &socket) {
               eprintln!("Errore: {:?}", err);
            } else {
            println!("Inserimento avvenuto con successo");
            }
        }
        if command=="list " {
            let hostname = args[1].to_string();
            if let Err(err) = client_list(&hostname, &socket) {
               eprintln!("Errore: {:?}", err);
            }
        }
    }
   
    Ok(())
    
}