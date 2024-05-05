use std::env;

fn main() {
    // Obtient le nom de l'utilisateur actuel
    let username = whoami::username();
    println!("Nom d'utilisateur : {}", username);

    // Obtient le nom de l'hôte
    let hostname = whoami::hostname();
    println!("Hôte : {}", hostname);

    // Obtient le type de système d'exploitation
    let os_type = env::consts::OS;
    println!("Système d'exploitation : {}", os_type);

}
