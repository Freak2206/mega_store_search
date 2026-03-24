use std::io;
use mega_store_search::catalogo::Catalogo;
use mega_store_search::recomendacao::Recomendacao;

fn main() {
   let mut catalogo = Catalogo::new();

// Eletrônicos
catalogo.adicionar("Notebook".to_string(), "Computador portátil".to_string(), 3999.00, "Eletrônicos".to_string());
catalogo.adicionar("Smartphone".to_string(), "Celular com 128GB".to_string(), 1999.90, "Eletrônicos".to_string());
catalogo.adicionar("Tablet".to_string(), "Tela 10 polegadas".to_string(), 1299.00, "Eletrônicos".to_string());
catalogo.adicionar("Fone Bluetooth".to_string(), "Fone sem fio com microfone".to_string(), 299.90, "Eletrônicos".to_string());

// Vestuário
catalogo.adicionar("Camiseta".to_string(), "Camiseta 100% algodão".to_string(), 49.90, "Vestuário".to_string());
catalogo.adicionar("Calça Jeans".to_string(), "Calça jeans azul masculina".to_string(), 159.90, "Vestuário".to_string());
catalogo.adicionar("Tênis".to_string(), "Tênis esportivo confortável".to_string(), 249.90, "Vestuário".to_string());

// Livros
catalogo.adicionar("Livro Rust".to_string(), "Aprenda programação em Rust".to_string(), 89.90, "Livros".to_string());
catalogo.adicionar("Livro Python".to_string(), "Guia completo de Python".to_string(), 99.90, "Livros".to_string());
catalogo.adicionar("Livro Inteligência Artificial".to_string(), "Conceitos e aplicações modernas".to_string(), 129.90, "Livros".to_string());

// Casa
catalogo.adicionar("Liquidificador".to_string(), "500W com copo de vidro".to_string(), 199.90, "Casa".to_string());
catalogo.adicionar("Cafeteira".to_string(), "Cafeteira elétrica 15 xícaras".to_string(), 249.90, "Casa".to_string());
catalogo.adicionar("Aspirador".to_string(), "Aspirador de pó portátil".to_string(), 399.90, "Casa".to_string());

    let mut recomendacoes = Recomendacao::new();

// Eletrônicos
recomendacoes.adicionar("Notebook", "Mouse");
recomendacoes.adicionar("Notebook", "Capa");
recomendacoes.adicionar("Smartphone", "Fone de ouvido");
recomendacoes.adicionar("Tablet", "Capa protetora");
recomendacoes.adicionar("Fone Bluetooth", "Carregador portátil");

// Vestuário
recomendacoes.adicionar("Camiseta", "Boné");
recomendacoes.adicionar("Calça Jeans", "Cinto de couro");
recomendacoes.adicionar("Tênis", "Meias esportivas");

// Livros
recomendacoes.adicionar("Livro Rust", "Livro Python");
recomendacoes.adicionar("Livro Python", "Livro Inteligência Artificial");
recomendacoes.adicionar("Livro Inteligência Artificial", "Livro Rust");

// Casa
recomendacoes.adicionar("Liquidificador", "Cafeteira");
recomendacoes.adicionar("Cafeteira", "Aspirador");
recomendacoes.adicionar("Aspirador", "Filtro de ar");

    loop {
        println!("\n=== Mega Store Search ===");
        println!("1 - Buscar por nome");
        println!("2 - Buscar por categoria");
        println!("3 - Buscar por faixa de preço");
        println!("4 - Ordenar por preço");
        println!("5 - Buscar por palavra-chave na descrição");
        println!("6 - Recomendações");
        println!("0 - Sair");
        println!("Escolha uma opção:");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).unwrap();
        let opcao = opcao.trim();

        match opcao {
            "1" => {
                println!("Digite o nome do produto:");
                let mut nome = String::new();
                io::stdin().read_line(&mut nome).unwrap();
                let nome = nome.trim();
                if let Some(p) = catalogo.buscar(nome) {
                    println!("{} - {} - R${} - Categoria: {}", p.nome, p.descricao, p.preco, p.categoria);
                } else {
                    println!("Produto não encontrado.");
                }
            }
            "2" => {
                println!("Digite a categoria:");
                let mut cat = String::new();
                io::stdin().read_line(&mut cat).unwrap();
                let cat = cat.trim();
                let lista = catalogo.buscar_por_categoria(cat);
                if lista.is_empty() {
                    println!("Nenhum produto encontrado na categoria '{}'.", cat);
                } else {
                    println!("Produtos da categoria '{}':", cat);
                    for p in lista {
                        println!("{} - {} - R${}", p.nome, p.descricao, p.preco);
                    }
                }
            }
            "3" => {
                println!("Digite o preço mínimo:");
                let mut min = String::new();
                io::stdin().read_line(&mut min).unwrap();
                let min: f32 = min.trim().parse().unwrap();

                println!("Digite o preço máximo:");
                let mut max = String::new();
                io::stdin().read_line(&mut max).unwrap();
                let max: f32 = max.trim().parse().unwrap();

                let lista = catalogo.buscar_por_preco(min, max);
                if lista.is_empty() {
                    println!("Nenhum produto encontrado na faixa de preço R${} - R${}.", min, max);
                } else {
                    println!("Produtos entre R${} e R${}:", min, max);
                    for p in lista {
                        println!("{} - {} - R${}", p.nome, p.descricao, p.preco);
                    }
                }
            }
            "4" => {
                println!("Ordenar por preço (crescente = 1, decrescente = 2):");
                let mut ordem = String::new();
                io::stdin().read_line(&mut ordem).unwrap();
                let crescente = ordem.trim() == "1";
                let lista = catalogo.ordenar_por_preco(crescente);
                println!("Produtos ordenados por preço:");
                for p in lista {
                    println!("{} - R${}", p.nome, p.preco);
                }
            }
            "5" => {
                println!("Digite a palavra-chave:");
                let mut chave = String::new();
                io::stdin().read_line(&mut chave).unwrap();
                let chave = chave.trim();
                let lista = catalogo.buscar_por_palavra_chave(chave);
                if lista.is_empty() {
                    println!("Nenhum produto contém '{}' na descrição.", chave);
                } else {
                    println!("Produtos que contém '{}' na descrição:", chave);
                    for p in lista {
                        println!("{} - {} - R${}", p.nome, p.descricao, p.preco);
                    }
                }
            }
            "6" => {
                println!("Digite o produto para ver recomendações:");
                let mut nome = String::new();
                io::stdin().read_line(&mut nome).unwrap();
                let nome = nome.trim();
                let lista = recomendacoes.sugerir(nome);
                if !lista.is_empty() {
                    println!("Quem compra {} também vê: {:?}", nome, lista);
                } else {
                    println!("Sem recomendações para este produto.");
                }
            }
            "0" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
}