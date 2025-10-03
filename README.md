# rust-portfolio-optimizer

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Nalgebra](https://img.shields.io/badge/nalgebra-math-red.svg?style=for-the-badge)
![Plotters](https://img.shields.io/badge/plotters-viz-purple.svg?style=for-the-badge)

**A high-performance portfolio optimization tool using Modern Portfolio Theory (MPT) in Rust.**

---

## 🇧🇷 Descrição em Português

`rust-portfolio-optimizer` é uma ferramenta de código aberto para otimização de portfólios de investimento baseada na Teoria Moderna do Portfólio (MPT) de Harry Markowitz. Desenvolvida em Rust, a ferramenta calcula a fronteira eficiente e encontra a alocação de ativos que maximiza o Índice de Sharpe.

Este é o terceiro de uma série de cinco repositórios focados em trading, mercado financeiro e IA, projetado para demonstrar a capacidade do Rust em computação numérica e otimização financeira.

### Funcionalidades

- **Otimização de Portfólio:** Implementa a simulação de Monte Carlo para encontrar a alocação ótima de ativos.
- **Cálculo de Risco e Retorno:** Calcula o retorno esperado e a volatilidade para diferentes combinações de portfólio.
- **Fronteira Eficiente:** Gera e visualiza a fronteira eficiente, mostrando o melhor retorno esperado para cada nível de risco.
- **Visualização de Dados:** Utiliza a biblioteca `plotters` para criar gráficos da fronteira eficiente.

---

## 🇺🇸 English Description

`rust-portfolio-optimizer` is an open-source tool for investment portfolio optimization based on Harry Markowitz's Modern Portfolio Theory (MPT). Developed in Rust, the tool calculates the efficient frontier and finds the asset allocation that maximizes the Sharpe Ratio.

This is the third in a series of five repositories focused on trading, the financial market, and AI, designed to demonstrate Rust's capabilities in numerical computing and financial optimization.

### Features

- **Portfolio Optimization:** Implements Monte Carlo simulation to find the optimal asset allocation.
- **Risk and Return Calculation:** Calculates the expected return and volatility for different portfolio combinations.
- **Efficient Frontier:** Generates and visualizes the efficient frontier, showing the best expected return for each level of risk.
- **Data Visualization:** Uses the `plotters` library to create charts of the efficient frontier.

---

## 🚀 Quick Start

### Pré-requisitos

- Rust (https://www.rust-lang.org/tools/install)
- Git

### Instalação

1. Clone o repositório:
```bash
git clone https://github.com/your-username/rust-portfolio-optimizer.git
cd rust-portfolio-optimizer
```

2. Compile e execute o exemplo:
```bash
cargo run --example portfolio_optimization
```

### Exemplo de Saída

O exemplo irá carregar os dados históricos de preços, executar a otimização e salvar um gráfico da fronteira eficiente.

```
Pesos Ótimos: [0.995, 0.004, 0.0002]
Gráfico da fronteira eficiente salvo em docs/efficient_frontier.png
```

O gráfico gerado (`docs/efficient_frontier.png`) se parecerá com este:

![Fronteira Eficiente](https://i.imgur.com/E5a7b8c.png)

---

## 🏛️ Arquitetura

O projeto é estruturado em um workspace do Cargo, com uma clara separação de responsabilidades entre os crates:

- `rpo-core`: Contém a lógica principal que orquestra a otimização.
- `rpo-data`: Responsável por carregar os dados de mercado.
- `rpo-optimizer`: Implementa os algoritmos de otimização e os cálculos de MPT.
- `rpo-utils`: Fornece utilitários, como a função de plotagem.

![Arquitetura do Otimizador](https://i.imgur.com/W9d0e1f.png)

---

## 🛣️ Roadmap

- [ ] Adicionar diferentes algoritmos de otimização (ex: Otimização por Gradiente).
- [ ] Incluir restrições de alocação (ex: pesos mínimos e máximos por ativo).
- [ ] Suporte para diferentes medidas de risco (ex: CVaR - Conditional Value at Risk).
- [ ] Integração com o `rust-market-data-pipeline` para obter dados em tempo real.
- [ ] Desenvolver uma interface de linha de comando (CLI) mais interativa.

---

## 🤝 Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir uma issue ou enviar um pull request.

---

## 📜 Licença

Este projeto está licenciado sob a licença MIT.

---

## 👨‍💻 Autor

**Gabriel Demetrios Lafis**

*   Cientista de Dados | Analista de Dados | BI/BA
*   Formado em Análise e Desenvolvimento de Sistemas, Gestão da Tecnologia da Informação e Segurança Cibernética.

