use std::process::Command;

pub fn install_dependencies() -> Result<(), std::io::Error> {
    // Atualiza a lista de pacotes do APT
    let output = Command::new("apt")
        .arg("update")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao atualizar a lista de pacotes do APT!"
        ));
    }

    // Instala as dependências do instalador
    let output = Command::new("apt")
        .arg("install")
        .arg("parted")
        .arg("e2fsprogs")
        .arg("debootstrap")
        .arg("openssl")
        .arg("-y")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao instalar as dependências do instalador!"
        ));
    }
    
    Ok(())
}