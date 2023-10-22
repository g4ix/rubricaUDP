use std::net::UdpSocket;
use std::str;
// creazione della struct persona
struct Persona {
    nome: String,
    cognome: String,
}
// il server salva le persone in un array
fn main() -> std::io::Result<()> {
    // creo la socket
    let socket = UdpSocket::bind("0.0.0.0:2000")?; 
    // buffer per la ricezione
    let mut buf = [0; 2048];
    // array di persone
    let mut persone: Vec<Persona> = Vec::new();

    loop {
        
        // Il metodo recv_from() restituisce una tupla con il numero di byte ricevuti e l'indirizzo del mittente
        let (amt, src) = socket.recv_from(&mut buf)?;
        // devo salvare la persona nell'array
        let recv = str::from_utf8(&buf[..amt]).unwrap();

        if recv == "list" {
            // invio la lista delle persone
            println!("Lista delle persone:");
            for persona in persone.iter() {
                println!("{} {}", persona.nome, persona.cognome);
            }
            continue;
        }

        println!("ricevuto: {}", recv);
        // splitto la stringa ricevuta in prossimità del carratere '+
        let split = recv.split('+');

        // estraggo il nome e il cognome con un iteratore
        let v: Vec<&str> = split.collect();
        let nome = v[0].to_owned();
        let cognome = v[1].to_owned();
        


        // inserisco la persona nell'array
        let persona = Persona {
            nome: nome,
            cognome: cognome,
        };
        persone.push(persona);


        // ridichiaro buf come slice di buf di dimensione amt che è il numero di byte ricevuti
        let buf = &mut buf[..amt];
        //invio la risposta al client
        socket.send_to(buf, &src)?;
    }
}