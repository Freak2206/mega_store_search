📦 Mega Store Search
Sistema de busca e recomendação de produtos desenvolvido em Rust, simulando funcionalidades básicas de um e‑commerce.

🚀 Funcionalidades
- Busca por nome → encontra produtos pelo nome exato.
- Busca por categoria → lista todos os produtos de uma categoria.
- Filtro por faixa de preço → mostra produtos dentro de um intervalo de valores.
- Ordenação por preço → lista produtos do mais barato ao mais caro (ou vice‑versa).
- Busca por palavra‑chave na descrição → encontra produtos que contenham termos específicos.
- Recomendações → sugere itens relacionados a um produto.
- Persistência em arquivo JSON → salva e carrega o catálogo de produtos.

🛠️ Tecnologias
- Linguagem: Rust (edition 2021)
- Gerenciador de pacotes: Cargo
- Bibliotecas:
- serde → serialização/desserialização
- serde_json → manipulação de arquivos JSON
- std::collections::HashMap → estrutura de dados para catálogo

📂 Estrutura do Projeto
AULA/
 ├── src/
 │    ├── main.rs
 │    ├── lib.rs
 │    ├── catalogo.rs
 │    └── recomendacao.rs
 ├── tests/
 │    ├── catalogo_tests.rs
 │    └── recomendacao_tests.rs
 ├── Cargo.toml
 └── README.md



▶️ Como rodar
- Instale o Rust: rust-lang.org/tools/install
- Clone ou abra o projeto no VS Code.
- No terminal, rode:
cargo run
- → Compila e executa o programa.
- Para rodar os testes:
cargo test



📌 Exemplo de saída
Produto encontrado: Notebook - Computador portátil - R$3999 - Categoria: Eletrônicos
Produtos da categoria Eletrônicos:
Notebook - R$3999
Smartphone - R$1999.9
Produtos até R$1000:
Camiseta - R$49.9
Produtos do mais barato ao mais caro:
Camiseta - R$49.9
Smartphone - R$1999.9
Notebook - R$3999
Produtos que contém 'algodão' na descrição:
Camiseta - Camiseta 100% algodão - R$49.9
Quem compra Notebook também vê: ["Mouse", "Capa"]



🎯 Objetivo
Este projeto foi desenvolvido como exercício prático para aprender:
- Estruturas de dados em Rust (HashMap, grafos).
- Organização de módulos e testes.
- Persistência de dados em JSON.
- Simulação de funcionalidades de e‑commerce.
