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
    let words: Vec<&str> = input.split("--").collect();

    // println!("words: {:?}", words);
    // words: ["insert ", "name Giulio ", "surname La Torre ", "server localhost"]

    // il comando è sempre il primo elemento
    let command = words[0].to_string();
    if command != "list " && command != "insert " {
        panic!("Comando non valido");
    }
    result.push(command);

    //inizializzo le stringhe come vuote
    let mut name = String::new();
    let mut surname = String::new();
    let mut hostname = String::new();

    // faccio un ciclo for sugli altri elementi e splitto per spazio
    for word in  words[1..].iter() {
        let pair: Vec<&str> = word.split(' ').collect(); // splitto per spazio
        
        if pair[0] == "name" {
            // devo riunire tutte le stringhe in una sola
            name = pair[1].to_string();
            for i in 2..pair.len()-1 {
                name.push(' ');
                name.push_str(pair[i]);
            }
        } else if pair[0] == "surname" {
            surname = pair[1].to_string();
            for i in 2..pair.len()-1 {
                surname.push(' ');
                surname.push_str(pair[i]);
            }
        } else if pair[0] == "server" {
            hostname = pair[1].to_string();
        } else {
            panic!("Comando non valido");
        }
    }
    result.push(name);
    result.push(surname);
    result.push(hostname);
    //println!("result: {:?}", result);
    result
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

        let args = split_line(&line);
       
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
            let hostname = args[3].to_string();
            if let Err(err) = client_list(&hostname, &socket) {
               eprintln!("Errore: {:?}", err);
            }
        }
    }
   
    Ok(())
    
}