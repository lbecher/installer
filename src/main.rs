mod configure;
mod configure_boot;
mod configure_storage;
mod constants;
mod dependencies;
mod install;

use std::env;
use std::process::exit;

use configure::*;
use configure_boot::*;
use configure_storage::*;
use dependencies::*;
use install::*;

fn main() {
    // Verifica se o usuário atual é o usuário root
    if let Ok(user) = env::var("USER") {
        if user != "root" {
            eprintln!("ERRO: Este programa precisa ser executado como root!");
            exit(1);
        }
    } else {
        eprintln!("ERRO: Não foi possível determinar o usuário atual!");
        exit(1);
    }

    // Obtém argumentos do terminal
    let args: Vec<String> = env::args().collect();

    // Verifica se o número de argumentos está correto
    if args.len() != 4 {
        eprintln!("\nUso: {} <emmc> <kernel> <dtb>\n\nOnde:\n", args[0]);
        eprintln!("  <emmc>    Caminho para dispositivo eMMC.\n            Exemplo: /dev/mmcblk0\n");
        eprintln!("  <kernel>  Caminho para o diretório do kernel Linux compilado.\n            Exemplo: /mnt/pendrive/linux-6.1.57\n");
        eprintln!("  <dtb>     Nome do arquivo em <kernel>/arch/arm/boot/dts que deve ser usado pelo kernel.\n            Exemplo: rk322x-box.dtb\n");
        exit(1);
    }

    // Obtém argumentos
    let storage_device_path = &args[1];
    let kernel_path = &args[2];
    let dtb_file = &args[3];

    // Verifica se o dispositivo informado é um eMMC válido
    if !storage_device_path.contains("mmcblk") {
        eprintln!("ERRO: O dispositivo informado não é um dispositivo eMMC válido!");
        exit(1);
    }

    // Obtém versão do kernel
    let release: String;
    match get_kernel_release(&kernel_path) {
        Ok(kernel_release) => {
            println!("As dependências do instalador foram instaladas com sucesso.");
            release = kernel_release;
        },
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }
    let kernel_release = release.as_str();

    // Obtém nome da máquina e senha do usuário root
    let hostname = get_hostname();
    let root_password = get_root_password();

    // Gera caminho para a partição raiz
    let root_partition_path = format!("{}p1", storage_device_path);


    // INSTALA DEPENDÊNCIAS DO INSTALADOR

    match install_dependencies() {
        Ok(()) => println!("As dependências do instalador foram instaladas com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }


    // CONFIGURA ARMAZENAMENTO

    match configure_storage(storage_device_path, &root_partition_path) {
        Ok(()) => println!("O dispositivo de armazenamento foi formatado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }


    // INSTALA O SISTEMA

    match create_root_filesystem() {
        Ok(()) => println!("O sistema de arquivos da raiz foi criado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }
    
    match prepare_root_filesystem() {
        Ok(()) => println!("O sistema de arquivos da raiz foi preparado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match install_kernel_modules(kernel_path) {
        Ok(()) => println!("Os módulos do kernel foram instalados com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }


    // CONFIGURA O SISTEMA

    match create_extlinux_configuration_file(&storage_device_path) {
        Ok(()) => println!("O arquivo /boot/extlinux/extlinux.conf foi criado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match set_hostname(&hostname) {
        Ok(()) => println!("O arquivo /etc/hostname foi criado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match set_hosts(&hostname) {
        Ok(()) => println!("O arquivo /etc/hosts foi criado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match set_fstab(&storage_device_path) {
        Ok(()) => println!("O arquivo /etc/fstab foi criado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match set_sources_list() {
        Ok(()) => println!("O arquivo /etc/apt/sources.list foi criado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match set_root_password(&root_password) {
        Ok(()) => println!("A senha do usuário root foi definida com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }


    // INSTALA PACOTES EXTRAS

    match install_extra_packages() {
        Ok(()) => println!("Os pacotes extras foram instalados no novo sistema com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }


    // CONFIGURA O BOOT

    match copy_boot_files(kernel_path, kernel_release) {
        Ok(()) => println!("Os arquivos de boot foram copiados com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match generate_boot_images(kernel_release) {
        Ok(()) => println!("As imagens de boot foram geradas com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match create_boot_symbolic_links(kernel_release, dtb_file) {
        Ok(()) => println!("O dispositivo de armazenamento foi desmontado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }


    // FINALIZA INSTALAÇÃO
    
    match umount_root_partition() {
        Ok(()) => println!("O dispositivo de armazenamento foi desmontado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }
}