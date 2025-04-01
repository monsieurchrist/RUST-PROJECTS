use std::net::TcpStream;
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;

/// Client pour le serveur de calcul géométrique
struct CalculatorClient {
    reader: BufReader<TcpStream>,
    stream: TcpStream,
}

impl CalculatorClient {
    /// Établit la connexion au serveur
    fn connect(addr: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))?; // Timeout de lecture
        
        let reader = BufReader::new(stream.try_clone()?);
        let mut client = Self { reader, stream };
        
        // Lecture immédiate du message de bienvenue
        let welcome = client.read_response()?;
        println!("{}", welcome);
        
        Ok(client)
    }

    /// Envoie une commande au serveur
    fn send_command(&mut self, command: &str) -> io::Result<()> {
        self.stream.write_all(format!("{}\n", command).as_bytes())?;
        self.stream.flush()
    }

    /// Lit la réponse du serveur
    fn read_response(&mut self) -> io::Result<String> {
        let mut response = String::new();
        self.reader.read_line(&mut response)?;
        Ok(response.trim().to_string())
    }

    /// Mode interactif avec l'utilisateur
    fn interactive_mode(&mut self) -> io::Result<()> {
        loop {
            // Affichage du menu
            println!("\n--- Menu Principal ---");
            println!("sr : Surface rectangle");
            println!("st : Surface triangle");
            println!("sc : Surface cercle");
            println!("ss : Surface sphère");
            println!("vp : Volume pavé");
            println!("vs : Volume sphère");
            println!("q  : Quitter");

            // Saisie de la commande
            print!("Votre choix > ");
            io::stdout().flush()?;
            
            let mut choice = String::new();
            io::stdin().read_line(&mut choice)?;
            let choice = choice.trim().to_lowercase();

            if choice == "q" {
                self.send_command("q")?;
                println!("Déconnexion...");
                return Ok(());
            }

            // Envoi de la commande et lecture du prompt
            self.send_command(&choice)?;
            let prompt = match self.read_response() {
                Ok(p) if p.starts_with("ERREUR") => {
                    println!("{}", p);
                    continue;
                }
                Ok(p) => p,
                Err(e) => return Err(e),
            };

            // Saisie des paramètres
            print!("{} > ", prompt);
            io::stdout().flush()?;
            
            let mut params = String::new();
            io::stdin().read_line(&mut params)?;
            
            // Envoi des paramètres et affichage du résultat
            self.send_command(params.trim())?;
            match self.read_response() {
                Ok(res) => println!("\n→ {}", res),
                Err(e) => eprintln!("Erreur: {}", e),
            }
        }
    }
}

/// Point d'entrée principal du client
fn main() {
    println!("Connexion au serveur...");
    match CalculatorClient::connect("addresse_du_serveur:6000") {
        Ok(mut client) => {
            if let Err(e) = client.interactive_mode() {
                eprintln!("Erreur: {}", e);
            }
        }
        Err(e) => eprintln!("Échec de connexion: {}", e),
    }
}
