use mega_store_search::catalogo::Catalogo;

#[test]
fn test_busca_produto() {
    let mut catalogo = Catalogo::new();
    catalogo.adicionar("Notebook".to_string(), "Computador portátil".to_string());

    assert_eq!(catalogo.buscar("Notebook"), Some(&"Computador portátil".to_string()));
    assert_eq!(catalogo.buscar("Smartphone"), None);
}