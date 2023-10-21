mod configure_system;
mod constants;
mod dependencies;
mod install_system;
mod storage;

use std::process::exit;

use configure_system::*;
use dependencies::*;
use install_system::*;
use storage::*;

fn main() {
    let hostname = get_hostname();
    let root_password = get_root_password();

    let storage_device_path = "/dev/mmcblk0";
    // Gera caminho para a partição raiz
    let root_partition_path = format!("{}p1", storage_device_path);

    match install_dependencies() {
        Ok(()) => println!("As dependências do instalador foram instaladas com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match config_storage_device(storage_device_path, &root_partition_path) {
        Ok(()) => println!("O dispositivo de armazenamento foi formatado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match create_root_filesystem() {
        Ok(()) => println!("O sistema de arquivos da raiz foi criado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }
    
    match prepare_system() {
        Ok(()) => println!("O sistema operacional foi preparado para ser usado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }

    match prepare_system() {
        Ok(()) => println!("O sistema operacional foi preparado para ser usado com sucesso."),
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

    match set_root_password(&root_password) {
        Ok(()) => println!("A senha do usuário root foi definida com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }
    
    match umount_root_partition() {
        Ok(()) => println!("O dispositivo de armazenamento foi desmontado com sucesso."),
        Err(error) => {
            eprintln!("ERRO: {}", error);
            exit(1);
        }
    }
}