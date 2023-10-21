use std::process::Command;

use crate::constants::*;

pub fn create_root_filesystem() -> Result<(), std::io::Error> {
    // Cria o sistema de arquivos da raiz
    let output = Command::new("debootstrap")
        .arg("--arch=armhf")
        .arg("--foreign")
        .arg("bookworm")
        .arg(ROOT_MOUNT_POINT)
        .arg("http://deb.debian.org/debian")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o sistema de arquivos da raiz!"
        ));
    }
    
    Ok(())
}

pub fn prepare_system() -> Result<(), std::io::Error> {
    // Prepara o sistema operacional para ser usado
    let output = Command::new("chroot")
        .arg(ROOT_MOUNT_POINT)
        .arg("/debootstrap/debootstrap")
        .arg("--second-stage")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao preparar o sistema operacional para ser usado!"
        ));
    }
    
    Ok(())
}