use std::fs;
use std::io;
use regex::Regex;

use std::process::Command;

use crate::constants::ROOT_MOUNT_POINT;

pub fn get_hostname() -> String {
    // Obtém nome da máquina
    let regex = Regex::new(r"^[a-zA-Z0-9.-]+$").unwrap();

    loop {
        let mut hostname = String::new();

        println!("Dê um nome para a sua máquina:");
        io::stdin().read_line(&mut hostname).expect("Erro ao ler a entrada do usuário!");

        hostname = hostname.trim().to_string();

        if regex.is_match(&hostname) {
            return hostname;
        } else {
            println!("Hostname inválido! Tente novamente.");
        }
    }
}

pub fn get_root_password() -> String {
    // Obtém senha do usuário root
    let mut password = String::new();

    loop {
        println!("Insira uma senha para o usuário root:");
        io::stdin().read_line(&mut password).expect("Erro ao ler a entrada do usuário!");
    
        password = password.trim().to_string();
    
        if password.is_empty() {
            println!("A senha não pode estar em branco!");
        } else if password.len() < 4 {
            println!("A senha não pode ter menos que quatro caracteres!");
        } else {
            break;
        }
    }

    password
}

pub fn set_hostname(hostname: &str) -> Result<(), std::io::Error>  {
    // Cria arquivo /etc/hostname
    let filepath = format!("{}/etc/hostname", ROOT_MOUNT_POINT);

    if let Err(_) = fs::write(filepath, hostname) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o arquivo /etc/hostname!"
        ));
    }

    Ok(())
}

pub fn set_hosts(hostname: &str) -> Result<(), std::io::Error>  {
    // Cria arquivo /etc/hosts
    let mut hosts = String::new();

    hosts += "127.0.0.1 localhost.localdomain localhost\n";
    hosts += "::1 localhost.localdomain localhost\n";
    hosts += &format!("127.0.1.1 {0}.localdomain {0}", hostname);

    let filepath = format!("{}/etc/hosts", ROOT_MOUNT_POINT);

    if let Err(_) = fs::write(filepath, hosts) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o arquivo /etc/hosts!"
        ));
    }

    Ok(())
}

pub fn set_fstab(storage_device_path: &str) -> Result<(), std::io::Error>  {
    // Cria arquivo /etc/fstab
    let fstab = format!("{}\text4\tdefaults\t0\t0", storage_device_path);

    let filepath = format!("{}/etc/fstab", ROOT_MOUNT_POINT);

    if let Err(_) = fs::write(filepath, fstab) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o arquivo /etc/fstab!"
        ));
    }

    Ok(())
}

pub fn set_root_password(root_password: &str) -> Result<(), std::io::Error>  {
    // Criptografa a senha do usuário root
    let output = Command::new("openssl")
        .arg("passwd")
        .arg("-1")
        .arg(root_password)
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criptografar a senha do usuário root!"
        ));
    }

    let encrypted_password = String::from_utf8(output.stdout).unwrap();

    // Define a senha do usuário root
    let output = Command::new("usermod")
        .arg("-p")
        .arg(encrypted_password)
        .arg("root")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao definir a senha do usuário root!"
        ));
    }

    Ok(())
}