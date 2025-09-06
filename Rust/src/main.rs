use std::io::{self, Write};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::process::Command;

pub static PRODUTOS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static STATUS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn voltar() {
    let mut entrada: String = String::new();
    print!("Digite qual quer coisa para voltar ao menu: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro no programa");
}

fn add() {
    Command::new("clear")
        .status()
        .unwrap();

    {
        let mut nome_produto: String = String::new();

        print!("Qual o nome do produto: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut nome_produto)
            .expect("Erro no programa");

        let mut produtos = PRODUTOS.lock().unwrap();
        let mut status = STATUS.lock().unwrap();

        produtos.push(nome_produto.to_string());
        status.push("Não vendido".to_string());
    }

    println!("Produto adicionado\n");
    voltar();
}

fn remover() {
    Command::new("clear")
        .status()
        .unwrap();

    let mut entrada: String = String::new();

    print!("Qual produto deseja remover: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro no programa");

    {
        let num_entrada: u16 = entrada.trim().parse().expect("Erro no programa");

        let mut produtos = PRODUTOS.lock().unwrap();
        let mut status = STATUS.lock().unwrap();

        produtos.remove((num_entrada - 1).into());
        status.remove((num_entrada - 1).into());
    }

    println!("Produto removido\n");
    voltar();
}

fn listar() {
    Command::new("clear")
        .status()
        .unwrap();

    {
        let produtos = PRODUTOS.lock().unwrap();
        let status = STATUS.lock().unwrap();

        for i in 0..produtos.len() {
            println!("{}: {}", produtos[i].trim(), status[i]);
        }

        println!();
        voltar();
    }
}

fn vender() {
    Command::new("clear")
        .status()
        .unwrap();

    {
        let mut entrada: String = String::new();

        print!("Qual produto deseja vender: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro no programa");

        let entrada_num: u32 = entrada.trim().parse().expect("Erro no programa");
        let mut status = STATUS.lock().unwrap();

        status[(entrada_num - 1) as usize] = "Vendido(a)".to_string();

        println!("Produto vendido com sucesso!!\n");
        voltar();
    }
}

fn main() {
    let mut entrada: String = String::new();
    let mut num_entrada: u8;

    while entrada.trim() != "0" {
        Command::new("clear")
            .status()
            .unwrap();

        println!("Adicionar um novo produto = 1");
        println!("Remover um produto = 2");
        println!("Listar produtos = 3");
        println!("Marcar produto como vendido = 4");
        println!("Sair = 0");

        print!("Digite o valor: ");
        io::stdout().flush().unwrap();

        entrada.clear();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro no programa");

        num_entrada = entrada.trim().parse().expect("Erro no programa");

        if num_entrada == 1 {
            add();
        } else if num_entrada == 2 {
            remover();
        } else if num_entrada == 3 {
            listar();
        } else if num_entrada == 4 {
            vender();
        }
    }
}