use std::process::Command;

pub fn install_dependencies() -> Result<(), std::io::Error> {
    // Atualiza o banco de dados do gerenciador de pacotes
    let output = Command::new("apt")
        .arg("update")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao atualizar o banco de dados do gerenciador de pacotes!"
        ));
    }

    // Instala as dependências do instalador
    let output = Command::new("apt")
        .arg("install")
        .arg("parted")
        .arg("e2fsprogs")
        .arg("debootstrap")
        .arg("openssl")
        .arg("build-essential")
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