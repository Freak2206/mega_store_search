use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use deunicode::deunicode; // para remover acentos

#[derive(Serialize, Deserialize, Debug)]
pub struct Produto {
    pub nome: String,
    pub descricao: String,
    pub preco: f32,
    pub categoria: String,
}

pub struct Catalogo {
    produtos: HashMap<String, Produto>,
}

impl Catalogo {
    pub fn new() -> Self {
        Catalogo {
            produtos: HashMap::new(),
        }
    }

    pub fn adicionar(&mut self, nome: String, descricao: String, preco: f32, categoria: String) {
        let produto = Produto {
            nome: nome.clone(),
            descricao,
            preco,
            categoria,
        };
        self.produtos.insert(nome, produto);
    }

    /// Função auxiliar para normalizar texto (remove acentos e coloca em minúsculas)
    fn normalizar(texto: &str) -> String {
        deunicode(texto).to_lowercase()
    }

    /// Busca por nome (case-insensitive e sem acento)
    pub fn buscar(&self, nome: &str) -> Option<&Produto> {
        let nome_norm = Self::normalizar(nome);
        self.produtos
            .values()
            .find(|p| Self::normalizar(&p.nome) == nome_norm)
    }

    /// Busca por categoria (case-insensitive e sem acento)
    pub fn buscar_por_categoria(&self, categoria: &str) -> Vec<&Produto> {
        let cat_norm = Self::normalizar(categoria);
        self.produtos
            .values()
            .filter(|p| Self::normalizar(&p.categoria) == cat_norm)
            .collect()
    }

    /// Busca por faixa de preço
    pub fn buscar_por_preco(&self, minimo: f32, maximo: f32) -> Vec<&Produto> {
        self.produtos
            .values()
            .filter(|p| p.preco >= minimo && p.preco <= maximo)
            .collect()
    }

    /// Ordenação por preço
    pub fn ordenar_por_preco(&self, crescente: bool) -> Vec<&Produto> {
        let mut lista: Vec<&Produto> = self.produtos.values().collect();
        if crescente {
            lista.sort_by(|a, b| a.preco.partial_cmp(&b.preco).unwrap());
        } else {
            lista.sort_by(|a, b| b.preco.partial_cmp(&a.preco).unwrap());
        }
        lista
    }

    /// Busca por palavra-chave na descrição (case-insensitive e sem acento)
    pub fn buscar_por_palavra_chave(&self, palavra: &str) -> Vec<&Produto> {
        let chave_norm = Self::normalizar(palavra);
        self.produtos
            .values()
            .filter(|p| Self::normalizar(&p.descricao).contains(&chave_norm))
            .collect()
    }

    /// Salvar catálogo em JSON
    pub fn salvar(&self, caminho: &str) {
        let json = serde_json::to_string_pretty(&self.produtos).unwrap();
        fs::write(caminho, json).unwrap();
    }

    /// Carregar catálogo de JSON
    pub fn carregar(&mut self, caminho: &str) {
        let dados = fs::read_to_string(caminho).unwrap();
        self.produtos = serde_json::from_str(&dados).unwrap();
    }
}