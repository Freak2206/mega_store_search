use mega_store_search::recomendacao::Recomendacao;

#[test]
fn test_recomendacao() {
    let mut rec = Recomendacao::new();
    rec.adicionar("Notebook", "Mouse");

    assert_eq!(rec.obter("Notebook"), Some(&vec!["Mouse".to_string()]));
}