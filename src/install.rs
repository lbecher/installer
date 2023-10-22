use std::process::Command;
use debug_print::debug_println;

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

pub fn prepare_root_filesystem() -> Result<(), std::io::Error> {
    // Prepara o sistema de arquivos da raiz
    let output = Command::new("chroot")
        .arg(ROOT_MOUNT_POINT)
        .arg("/debootstrap/debootstrap")
        .arg("--second-stage")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao preparar o sistema de arquivos da raiz!"
        ));
    }
    
    Ok(())
}

pub fn install_extra_packages() -> Result<(), std::io::Error> {
    // Atualiza o banco de dados do gerenciador de pacotes do novo sistema
    let output = Command::new("chroot")
        .arg(ROOT_MOUNT_POINT)
        .arg("/bin/apt")
        .arg("update")
        .output()?;

    debug_println!("{:?}", output);
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao atualizar a lista de pacotes do APT do novo sistema!"
        ));
    }

    //  Instala pacotes extras no novo sistema
    let output = Command::new("chroot")
        .arg(ROOT_MOUNT_POINT)
        .arg("/bin/apt")
        .arg("install")
        .arg("u-boot-tools")
        .arg("initramfs-tools")
        .arg("udev")
        .arg("wget")
        .arg("curl")
        .arg("ntpdate")
        .arg("sudo")
        .arg("iputils-ping")
        .arg("network-manager")
        .arg("openssh-server")
        .arg("dosfstools")
        .arg("tar")
        .arg("zip")
        .arg("unzip")
        .arg("-y")
        .output()?;

    debug_println!("{:?}", output);
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao instalar pacotes extras no novo sistema!"
        ));
    }

    Ok(())
}

pub fn install_kernel_modules(
    kernel_path: &str
) -> Result<(), std::io::Error>  {
    // Instala módulos do kernel
    let output = Command::new("make")
        .arg("-s")
        .arg("-C")
        .arg(kernel_path)
        .arg(format!("INSTALL_MOD_DIR={}", ROOT_MOUNT_POINT))
        .arg("modules_install")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao instalar módulos do kernel!"
        ));
    }

    Ok(())
}