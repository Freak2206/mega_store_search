use mega_store_search::catalogo::Catalogo;

#[test]
fn test_busca_produto() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar("Notebook".to_string(), "Computador portátil".to_string(), 3999.0, "Eletrônicos".to_string());

    let produto = catalogo.buscar("Notebook");
    assert!(produto.is_some());
    assert_eq!(produto.unwrap().descricao, "Computador portátil");

    assert_eq!(catalogo.buscar("Smartphone"), None);
}

#[test]
fn test_busca_por_categoria() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar("Camiseta".to_string(), "Algodão".to_string(), 49.9, "Vestuário".to_string());
    catalogo.adicionar("Tênis".to_string(), "Esportivo".to_string(), 249.9, "Vestuário".to_string());

    let resultados = catalogo.buscar_por_categoria("vestuario");
    assert_eq!(resultados.len(), 2);
}

#[test]
fn test_busca_por_preco() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar("Livro Rust".to_string(), "Aprenda Rust".to_string(), 89.9, "Livros".to_string());
    catalogo.adicionar("Notebook".to_string(), "Computador portátil".to_string(), 3999.0, "Eletrônicos".to_string());

    let baratos = catalogo.buscar_por_preco(0.0, 100.0);
    assert_eq!(baratos.len(), 1);
    assert_eq!(baratos[0].nome, "Livro Rust");
}

#[test]
fn test_ordenar_por_preco() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar("Livro Rust".to_string(), "Aprenda Rust".to_string(), 89.9, "Livros".to_string());
    catalogo.adicionar("Notebook".to_string(), "Computador portátil".to_string(), 3999.0, "Eletrônicos".to_string());

    let ordenados = catalogo.ordenar_por_preco(true);
    assert_eq!(ordenados[0].nome, "Livro Rust");
    assert_eq!(ordenados[1].nome, "Notebook");
}

#[test]
fn test_busca_por_palavra_chave() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar("Camiseta".to_string(), "Camiseta 100% algodão".to_string(), 49.9, "Vestuário".to_string());

    let resultados = catalogo.buscar_por_palavra_chave("algodao");
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].nome, "Camiseta");
}