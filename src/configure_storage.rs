use std::process::Command;

use crate::constants::*;

pub fn configure_storage(
    storage_device_path: &str,
    root_partition_path: &str
) -> Result<(), std::io::Error> {
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