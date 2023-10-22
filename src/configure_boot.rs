use std::fs;
use std::process::Command;

use crate::constants::*;

pub fn create_extlinux_configuration_file(
    storage_device_path: &str
) -> Result<(), std::io::Error> {
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
    extlinux += "  FDT ../device_tree_binary.dtb\n";
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

pub fn copy_boot_files(
    kernel_path: &str,
    kernel_release: &str
) -> Result<(), std::io::Error> {
    // Cria o caminho /boot/dtb-<kernel_release>
    let output = Command::new("mkdir")
        .arg("-p")
        .arg(format!("{}/boot/dtb-{}", ROOT_MOUNT_POINT, kernel_release))
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Falha ao criar o caminho /boot/dtb-{}!", kernel_release)
        ));
    }

    // Copia os arquivos DTB
    let output = Command::new("cp")
        .arg(format!("{}/arch/arm/boot/dts/*.dtb", kernel_path))
        .arg(format!("{}/boot/dtb-{}", ROOT_MOUNT_POINT, kernel_release))
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao copiar os arquivos DTB!"
        ));
    }

    // Copia o arquivo zImage
    let output = Command::new("cp")
        .arg(format!("{}/arch/arm/boot/zImage", kernel_path))
        .arg(format!("{}/boot/zImage-{}", ROOT_MOUNT_POINT, kernel_release))
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao copiar o arquivo zImage!"
        ));
    }

    // Copia o arquivo System.map
    let output = Command::new("cp")
        .arg(format!("{}/System.map", kernel_path))
        .arg(format!("{}/boot/System.map-{}", ROOT_MOUNT_POINT, kernel_release))
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao copiar o arquivo System.map!"
        ));
    }

    Ok(())
}

pub fn generate_boot_images(
    kernel_release: &str,
) -> Result<(), std::io::Error> {
    // Gera imagem uImage
    let output = Command::new("mkimage")
        .arg("-A")
        .arg("arm")
        .arg("-O")
        .arg("linux")
        .arg("-T")
        .arg("kernel")
        .arg("-C")
        .arg("none")
        .arg("-a")
        .arg("0x600f0000")
        .arg("-e")
        .arg("0x600f0000")
        .arg("-n")
        .arg(kernel_release)
        .arg("-d")
        .arg(format!("{}/boot/zImage-{}", ROOT_MOUNT_POINT, kernel_release))
        .arg(format!("{}/boot/uImage-{}", ROOT_MOUNT_POINT, kernel_release))
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao gerar imagem uImage!"
        ));
    }

    // Gera imagem initrd.img
    let output = Command::new("update-initramfs")
        .arg("-c")
        .arg("-k")
        .arg(kernel_release)
        .arg("-b")
        .arg(format!("{}/boot", ROOT_MOUNT_POINT))
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao gerar imagem initrd.img!"
        ));
    }

    // Gera imagem uInitrd
    let output = Command::new("mkimage")
        .arg("-A")
        .arg("arm")
        .arg("-O")
        .arg("linux")
        .arg("-T")
        .arg("ramdisk")
        .arg("-a")
        .arg("0x0")
        .arg("-e")
        .arg("0x0")
        .arg("-n")
        .arg(format!("initrd.img-{}", kernel_release))
        .arg("-d")
        .arg(format!("{}/boot/initrd.img-{}", ROOT_MOUNT_POINT, kernel_release))
        .arg(format!("{}/boot/uInitrd-{}", ROOT_MOUNT_POINT, kernel_release))
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao gerar imagem uInitrd!"
        ));
    }

    Ok(())
}

pub fn create_boot_symbolic_links(
    kernel_release: &str,
    dtb_file: &str
) -> Result<(), std::io::Error> {
    // Cria o link simbólico para zImage
    let output = Command::new("chroot")
        .arg(ROOT_MOUNT_POINT)
        .arg("/bin/ln")
        .arg("-s")
        .arg(format!("/boot/zImage-{}", kernel_release))
        .arg("/boot/zImage")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar link simbólico para zImage!"
        ));
    }

    // Cria o link simbólico para initrd.img
    let output = Command::new("chroot")
        .arg(ROOT_MOUNT_POINT)
        .arg("/bin/ln")
        .arg("-s")
        .arg(format!("/boot/initrd.img-{}", kernel_release))
        .arg("/boot/initrd.img")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o link simbólico para initrd.img!"
        ));
    }

    // Cria o link simbólico para o diretório dtb
    let output = Command::new("chroot")
        .arg(ROOT_MOUNT_POINT)
        .arg("/bin/ln")
        .arg("-s")
        .arg(format!("/boot/dtb-{}", kernel_release))
        .arg("/boot/dtb")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o link simbólico para o diretório dtb!"
        ));
    }

    // Cria o link simbólico para device_tree_binary.dtb
    let output = Command::new("chroot")
        .arg(ROOT_MOUNT_POINT)
        .arg("/bin/ln")
        .arg("-s")
        .arg(format!("/boot/dtb/{}", dtb_file))
        .arg("/boot/device_tree_binary.dtb")
        .output()?;
    
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao criar o link simbólico para device_tree_binary.dtb!"
        ));
    }
    
    Ok(())
}