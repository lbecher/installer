use std::fs;
use std::io;
use regex::Regex;

use std::process::Command;

use crate::constants::*;

pub fn get_hostname() -> String {
    // Obtém o nome da máquina
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
    // Obtém a senha do usuário root
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
    // Cria o arquivo /etc/hostname
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
    // Cria o arquivo /etc/hosts
    let mut hosts = String::new();

    hosts += "127.0.0.1 localhost.localdomain localhost\n";
    hosts += "::1 localhost.localdomain localhost\n";
    hosts += format!("127.0.1.1 {0}.localdomain {0}", hostname).as_str();

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
    // Cria o arquivo /etc/fstab
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

pub fn set_sources_list() -> Result<(), std::io::Error> {
    // Cria o arquivo /etc/apt/sources.list
    let mut sources_list = String::new();

    sources_list += "deb http://deb.debian.org/debian bookworm main non-free non-free-firmware\n";
    sources_list += "deb-src http://deb.debian.org/debian bookworm main non-free non-free-firmware\n";
    sources_list += "\n";
    sources_list += "deb http://deb.debian.org/debian-security/ bookworm-security main non-free non-free-firmware\n";
    sources_list += "deb-src http://deb.debian.org/debian-security/ bookworm-security main non-free non-free-firmware\n";
    sources_list += "\n";
    sources_list += "deb http://deb.debian.org/debian bookworm-updates main non-free non-free-firmware\n";
    sources_list += "deb-src http://deb.debian.org/debian bookworm-updates main non-free non-free-firmware\n";

    let filepath = format!("{}/etc/apt/sources.list", ROOT_MOUNT_POINT);

    if let Err(_) = fs::write(filepath, sources_list) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o arquivo /etc/apt/sources.list!"
        ));
    }

    Ok(())
}

pub fn set_extlinux(storage_device_path: &str) -> Result<(), std::io::Error> {
    // Cria o caminho /boot/extlinux
    let output = Command::new("mkdir")
        .arg("-p")
        .arg(format!("{}/boot/extlinux", ROOT_MOUNT_POINT).as_str())
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o caminho /boot/extlinux!"
        ));
    }

    // Cria o arquivo /boot/extlinux/extlinux.conf
    let mut extlinux = String::new();

    extlinux += "LABEL Linux\n";
    extlinux += "  LINUX ../zImage\n";
    extlinux += "  INITRD ../initrd.img\n";
    extlinux += "  FDT ../deviceTreeBinary.dtb\n";
    extlinux += format!(
        "  APPEND earlyprintk root={} rootwait rootfstype=ext4 init=/sbin/init\n", 
        storage_device_path).as_str();

    let filepath = format!("{}/boot/extlinux/extlinux.conf", ROOT_MOUNT_POINT);

    if let Err(_) = fs::write(filepath, extlinux) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o arquivo /boot/extlinux/extlinux.conf!"
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

    let encrypted_password = String::from_utf8(output.stdout)
        .unwrap()
        .replace("\n", "");

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

pub fn config_storage_device(storage_device_path: &str, root_partition_path: &str) -> Result<(), std::io::Error> {
    // Executa o comando para criar uma tabela de partição MBR
    let output = Command::new("parted")
        .arg("--script")
        .arg(storage_device_path)
        .arg("mklabel")
        .arg("msdos")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar a tabela de partição MBR!"
        ));
    }
    
    // Executa o comando para criar uma partição raiz
    let output = Command::new("parted")
        .arg("--script")
        .arg(storage_device_path)
        .arg("mkpart")
        .arg("primary")
        .arg("ext4")
        .arg("0%")
        .arg("100%")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other, 
            "Falha ao criar a partição raiz!"
        ));
    }

    // Executa o comando para tornar a partição raiz inicializável
    let output = Command::new("parted")
        .arg("--script")
        .arg(storage_device_path)
        .arg("set")
        .arg("1")
        .arg("boot")
        .arg("on")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other, 
            "Falha ao tornar a partição raiz inicializável!"
        ));
    }
    
    // Formata a partição raiz com EXT4
    let output = Command::new("mkfs.ext4")
        .arg(&root_partition_path)
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao formatar a partição raiz com EXT4!"
        ));
    }

    // Cria o ponto de montagem para a partição raiz
    let output = Command::new("mkdir")
        .arg("-p")
        .arg(ROOT_MOUNT_POINT)
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o ponto de montagem para a partição raiz!"
        ));
    }

    // Monta a partição raiz
    let output = Command::new("mount")
        .arg(&root_partition_path)
        .arg(ROOT_MOUNT_POINT)
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao montar a partição raiz!"
        ));
    }
    
    Ok(())
}

pub fn umount_root_partition() -> Result<(), std::io::Error> {
    // Desmonta a partição raiz
    let output = Command::new("umount")
        .arg(ROOT_MOUNT_POINT)
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao desmontar a partição raiz!"
        ));
    }
    
    Ok(())
}