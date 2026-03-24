use mega_store_search::recomendacao::Recomendacao;

#[test]
fn test_recomendacoes() {
    let mut rec = Recomendacao::new();
    rec.adicionar("Notebook", "Mouse");
    rec.adicionar("Notebook", "Capa");

    let sugestoes = rec.sugerir("Notebook");
    assert_eq!(sugestoes.len(), 2);
    assert!(sugestoes.contains(&"Mouse".to_string()));
    assert!(sugestoes.contains(&"Capa".to_string()));

    let vazio = rec.sugerir("Smartphone");
    assert_eq!(vazio.len(), 0);
}