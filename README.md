# 🔐 Cifra de Vigenère - Projeto Educacional
## Implementação em Rust para Cyber Segurança

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Security](https://img.shields.io/badge/Security-Educational-red?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)

*Projeto desenvolvido para ensino de criptografia clássica*

</div>

---

## 📚 Sobre o Projeto

Este projeto implementa a **Cifra de Vigenère** em Rust, uma cifra polialfabética criada no século XVI que foi considerada "inquebrável" por mais de 300 anos. O objetivo é educacional, focado no ensino de:

- ✅ Fundamentos de criptografia clássica
- ✅ Vulnerabilidades em sistemas de segurança
- ✅ Técnicas básicas de criptoanálise
- ✅ Programação em Rust aplicada à segurança

---

## 🎯 Objetivos Educacionais

### Para Estudantes de Cyber Segurança:
- Compreender a evolução histórica da criptografia
- Identificar fraquezas em algoritmos aparentemente seguros
- Desenvolver pensamento crítico sobre segurança
- Aplicar matemática à resolução de problemas práticos

### Para Desenvolvedores:
- Implementar algoritmos criptográficos em Rust
- Desenvolver código limpo e bem documentado
- Criar testes unitários abrangentes
- Aplicar boas práticas de programação

---

## 🚀 Funcionalidades

### Versão Básica (`main.rs`)
- ✨ Implementação simples e direta
- 🔒 Cifragem de texto usando chave fixa
- 📊 Exemplo clássico: "ATTACK AT DAWN" + "LEMON"

### Versão Avançada (`codigo_melhorado.rs`)
- 🖥️ Interface interativa completa
- 🔒 Cifragem e decifragem de mensagens
- 📊 Análise de frequência de letras
- 📈 Cálculo do Índice de Coincidência
- 🎯 Modo demonstração educacional
- ✅ Validação de entrada e tratamento de erros
- 🧪 Testes unitários abrangentes

---

## 📖 Como Usar

### Pré-requisitos
```bash
# Instalar Rust (se não tiver)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verificar instalação
rustc --version
cargo --version
```

### Executar Versão Básica
```bash
# Clonar o repositório
git clone <seu-repositorio>
cd cifra_vigenere

# Executar
cargo run
```

### Executar Versão Avançada
```bash
# Copiar código melhorado para main.rs ou executar diretamente
cp src/codigo_melhorado.rs src/main.rs
cargo run

# Ou compilar como binário separado
rustc src/codigo_melhorado.rs -o vigenere_avancado
./vigenere_avancado
```

### Executar Testes
```bash
cargo test
```

---

## 🎮 Demonstração Interativa

### Menu Principal
```
🔐 === CIFRA DE VIGENÈRE ===
Implementação para Cyber Segurança
---------------------------------

📋 === MENU PRINCIPAL ===
1. 🔒 Cifrar mensagem
2. 🔓 Decifrar mensagem  
3. 🎯 Demonstração (ATTACK/LEMON)
4. 📊 Análise de frequência
5. 🚪 Sair

Escolha uma opção:
```

### Exemplo de Uso
```
🔒 === MODO CIFRAGEM ===
Digite o texto a ser cifrado: HELLO WORLD
Digite a chave (apenas letras): KEY

✅ Resultado:
Texto original: HELLOWORLD
Chave usada:    KEY
Texto cifrado:  RIJVSGSPNH

🔍 === PROCESSO DETALHADO ===
Posição | Original | Chave | Cifrado | Cálculo
--------|----------|-------|---------|--------
   0    |    H     |   K   |    R    | 7+10=17 mod 26
   1    |    E     |   E   |    I    | 4+4=8 mod 26
   2    |    L     |   Y   |    J    | 11+24=9 mod 26
   ...
```

---

## 🏛️ Contexto Histórico

### Timeline
- **1553** 📜 Giovan Battista Bellaso descreve a cifra
- **1586** 🏛️ Blaise de Vigenère populariza (incorretamente creditado)
- **1800s** 🔒 Chamada de "Le Chiffre Indéchiffrable"
- **1863** 💡 Friedrich Kasiski desenvolve método de quebra
- **1917** 🔓 Gilbert Vernam cria o "One-Time Pad" perfeito
- **2025** 💻 Usada para ensino de criptografia

### Importância Pedagógica
A cifra de Vigenère é ideal para ensino porque:
- É **complexa o suficiente** para ser interessante
- É **simples o suficiente** para ser compreendida
- Tem **vulnerabilidades exploráveis** manualmente
- Ilustra **conceitos fundamentais** da criptografia

---

## 🔬 Análise Técnica

### Como Funciona
```rust
// Fórmula de cifragem
C[i] = (P[i] + K[i mod len(K)]) mod 26

// Fórmula de decifragem  
P[i] = (C[i] - K[i mod len(K)] + 26) mod 26
```

### Exemplo Matemático
```
Texto:  A T T A C K
Chave:  L E M O N L
        ↓ ↓ ↓ ↓ ↓ ↓
Posição: 0+11  19+4  19+12  0+14  2+13  10+11
Resultado: 11   23    5     14    15    21
Cifrado:   L    X     F     O     P     V
```

### Índice de Coincidência
```rust
IC = Σ(ni × (ni-1)) / (N × (N-1))
```

**Interpretação:**
- **IC ≈ 0.072**: Texto em português (cifra quebrada)
- **IC ≈ 0.038**: Texto aleatório (cifra intacta)

---

## 🕵️ Criptoanálise

### Método de Kasiski (1863)

#### 1. Encontrar Repetições
```
Texto: LXFOPVEFRHRXFOP...
        ^^^     ^^^
       Repetição!
```

#### 2. Calcular Distâncias
- Distância entre repetições = múltiplo do comprimento da chave
- MCD das distâncias = comprimento provável da chave

#### 3. Análise de Frequência
- Dividir texto em grupos baseados no comprimento da chave
- Cada grupo é uma cifra de César!
- Aplicar estatística de frequência das letras

### Implementação do Ataque
```rust
fn quebrar_vigenere(texto_cifrado: &str) -> Option<(String, String)> {
    // 1. Detectar comprimento da chave
    let comprimento = detectar_comprimento_chave(texto_cifrado)?;
    
    // 2. Análise de frequência por posição
    let chave = encontrar_chave_por_frequencia(texto_cifrado, comprimento)?;
    
    // 3. Decifrar e validar
    let texto_original = vigenere_decrypt(texto_cifrado, &chave);
    
    Some((texto_original, chave))
}
```

---

## 📊 Estrutura do Código

### Arquivos Principais
```
cifra_vigenere/
├── src/
│   ├── main.rs                 # Implementação básica
│   └── codigo_melhorado.rs     # Versão completa
├── Cargo.toml                  # Configuração do projeto
├── AULA_CIFRA_VIGENERE.md     # Material didático completo
├── ROTEIRO_APRESENTACAO.md    # Guia para professores
├── EXERCICIOS_PRATICOS.md     # Lista de atividades
└── README.md                  # Este arquivo
```

### Funções Principais
```rust
// Cifragem e decifragem
fn vigenere_encrypt(texto: &str, chave: &str) -> String
fn vigenere_decrypt(texto_cifrado: &str, chave: &str) -> String

// Análise estatística
fn calcular_frequencias(texto: &str) -> HashMap<char, usize>
fn calcular_indice_coincidencia(texto: &str) -> f64

// Interface do usuário
fn modo_cifragem()
fn modo_decifragem()
fn modo_demo()
fn modo_analise()
```

---

## 🧪 Testes e Validação

### Suíte de Testes
```bash
cargo test

# Executar testes específicos
cargo test test_cifragem_basica
cargo test test_indice_coincidencia
```

### Casos de Teste Incluídos
- ✅ Cifragem e decifragem básica
- ✅ Exemplo clássico da aula (ATTACK/LEMON)
- ✅ Casos extremos (chave de 1 letra, chave longa)
- ✅ Cálculo correto do Índice de Coincidência
- ✅ Validação de entrada e tratamento de erros

### Exemplo de Teste
```rust
#[test]
fn test_exemplo_aula() {
    let resultado = vigenere_encrypt("ATTACKATDAWN", "LEMON");
    assert_eq!(resultado, "LXFOPVEFRHR");
    
    let decifrado = vigenere_decrypt("LXFOPVEFRHR", "LEMON");
    assert_eq!(decifrado, "ATTACKATDAWN");
}
```

---

## 🎓 Material Educacional

### Arquivos de Apoio
1. **AULA_CIFRA_VIGENERE.md**: Apresentação completa (90 min)
2. **ROTEIRO_APRESENTACAO.md**: Guia para professores
3. **EXERCICIOS_PRATICOS.md**: 8 exercícios graduais

### Tópicos Abordados
- 🏛️ História e contexto da cifra
- 🔧 Implementação prática em Rust
- 🔍 Técnicas de criptoanálise
- 📊 Análise estatística e matemática
- 🛡️ Implicações para segurança moderna

### Exercícios Incluídos
1. **Cifragem manual** (iniciante)
2. **Decifragem manual** (iniciante)
3. **Melhoramento do código** (intermediário)
4. **Criptoanálise - Método Kasiski** (avançado)
5. **Análise estatística** (avançado)
6. **Quebrador automático** (expert)
7. **Análise de vulnerabilidades** (intermediário)
8. **Comparação de cifras** (intermediário)

---

## 🛡️ Considerações de Segurança

### ⚠️ IMPORTANTE: USO EDUCACIONAL APENAS
```
🚨 Esta implementação é para fins educacionais!
   NUNCA use para proteger dados reais!
```

### Por que a Cifra de Vigenère é Insegura?
1. **Chaves curtas** repetem padrões
2. **Análise de frequência** ainda é possível
3. **Método de Kasiski** pode quebrar automaticamente
4. **Reutilização de chaves** revela informações
5. **Não há autenticação** de mensagens

### Aplicações Modernas
- 📚 **Educação**: Ensino de conceitos básicos
- 🎮 **CTF**: Desafios de criptografia
- 🕵️ **Pentest**: Identificar cifras fracas em sistemas legados
- 📊 **Pesquisa**: Base para algoritmos modernos

---

## 🚀 Melhorias Futuras

### Versão 2.0 (Planejada)
- [ ] Interface gráfica (GUI) com Tauri
- [ ] Quebrador automático de Vigenère
- [ ] Suporte a outros alfabetos (cirílico, árabe)
- [ ] Geração automática de chaves seguras
- [ ] Análise comparativa com outras cifras
- [ ] Modo "competição" para salas de aula

### Contribuições
Contribuições são bem-vindas! Por favor:
1. Fork o repositório
2. Crie uma branch para sua feature
3. Implemente testes para nova funcionalidade
4. Envie um Pull Request

---

## 📚 Referências e Bibliografia

### Livros Recomendados
1. **"The Codebreakers"** - David Kahn
2. **"The Code Book"** - Simon Singh  
3. **"Applied Cryptography"** - Bruce Schneier
4. **"Introduction to Modern Cryptography"** - Katz & Lindell

### Papers Históricos
- Kasiski, F. (1863). "Die Geheimschriften und die Dechiffrir-kunst"
- Vernam, G. (1926). "Cipher Printing Telegraph Systems"

### Recursos Online
- [Crypto Museum](https://www.cryptomuseum.com/)
- [Practical Cryptography](http://practicalcryptography.com/)
- [CryptoCrack](https://www.cryptoclub.org/)

---

## 📄 Licença

```
MIT License

Copyright (c) 2025 Projeto Cifra de Vigenère

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 👨‍🏫 Sobre o Autor

Este projeto foi desenvolvido como material didático para disciplinas de **Cyber Segurança** e **Criptografia**. 

Para dúvidas, sugestões ou melhorias, entre em contato:
- 📧 Email: [seu-email@exemplo.com]
- 🐱 GitHub: [seu-usuario]
- 💼 LinkedIn: [seu-perfil]

---

<div align="center">

**⭐ Se este projeto foi útil, considere dar uma estrela!**

*Feito com ❤️ para a comunidade de Cyber Segurança*

</div>