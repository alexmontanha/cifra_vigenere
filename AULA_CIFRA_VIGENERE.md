# Aula: Cifra de Vigenère
## Disciplina: Cyber Segurança

---

## 📚 Sumário
1. [Introdução](#introdução)
2. [História](#história)
3. [Como Funciona](#como-funciona)
4. [Implementação Prática](#implementação-prática)
5. [Vantagens e Desvantagens](#vantagens-e-desvantagens)
6. [Criptoanálise](#criptoanálise)
7. [Exercícios Práticos](#exercícios-práticos)
8. [Referências](#referências)

---

## 🎯 Objetivos da Aula

Ao final desta aula, você será capaz de:
- ✅ Compreender o funcionamento da cifra de Vigenère
- ✅ Implementar o algoritmo de cifragem e decifragem
- ✅ Identificar vulnerabilidades da cifra
- ✅ Aplicar técnicas básicas de criptoanálise
- ✅ Comparar com outras cifras clássicas

---

## 🏛️ História

### Origens
- **Século XVI**: Primeira descrição por **Giovan Battista Bellaso** (1553)
- **1586**: Falsamente atribuída a **Blaise de Vigenère**
- **Apelido**: "Le Chiffre Indéchiffrable" (A Cifra Indecifrável)
- **Quebrada**: Apenas em 1863 por **Friedrich Kasiski**

### Importância Histórica
- Usada por confederados na Guerra Civil Americana
- Empregada em comunicações diplomáticas
- Base para cifras mais modernas (como Vernam)

---

## 🔧 Como Funciona

### Conceito Básico
A cifra de Vigenère é uma **cifra polialfabética** que usa uma palavra-chave para determinar o deslocamento de cada letra do texto claro.

### Algoritmo Matemático
```
C[i] = (P[i] + K[i mod len(K)]) mod 26
```

Onde:
- `C[i]` = i-ésimo caractere do texto cifrado
- `P[i]` = i-ésimo caractere do texto claro
- `K[i mod len(K)]` = caractere da chave (repetida ciclicamente)

### Tabela de Vigenère (Tabula Recta)
```
    A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
A   A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
B   B C D E F G H I J K L M N O P Q R S T U V W X Y Z A
C   C D E F G H I J K L M N O P Q R S T U V W X Y Z A B
D   D E F G H I J K L M N O P Q R S T U V W X Y Z A B C
... (continua para todas as letras)
```

### Exemplo Prático
**Texto Claro**: `ATTACKATDAWN`
**Chave**: `LEMON`
**Chave Repetida**: `LEMONLEMONLE`

```
Texto:  A T T A C K A T D A W N
Chave:  L E M O N L E M O N L E
Soma:   L X F O P V E F R N H R
```

**Resultado**: `LXFOPVEFRHR`

---

## 💻 Implementação Prática

### Código em Rust (Análise do Projeto)

```rust
fn main() {
    let plaintext = "ATTACKATDAWN";
    let key = "LEMON";
    let ciphertext = vigenere(plaintext, key);
    println!("{}", ciphertext);
}

fn vigenere(p0: &str, p1: &str) -> String {
    let mut ciphertext = String::new();
    let mut i = 0;
    
    for c in p0.chars() {
        let k = p1.chars().nth(i).unwrap();
        let c = shift(c, k);
        ciphertext.push(c);
        i = (i + 1) % p1.len();
    }
    
    ciphertext
}

fn shift(p0: char, p1: char) -> char {
    let a = 'A' as u8;
    let p0 = p0 as u8;
    let p1 = p1 as u8;
    let c = ((p0 - a) + (p1 - a)) % 26 + a;
    c as char
}
```

### Análise do Código
1. **Função `main()`**: Define o texto claro e a chave
2. **Função `vigenere()`**: Aplica a cifra caractere por caractere
3. **Função `shift()`**: Realiza o deslocamento matemático
4. **Operação modular**: Garante que o resultado permaneça no alfabeto

---

## ⚖️ Vantagens e Desvantagens

### ✅ Vantagens
- **Polialfabética**: Mais segura que cifras monoalfabéticas
- **Simplicidade**: Fácil de implementar e ensinar
- **Resistência à análise de frequência simples**
- **Chave reutilizável**

### ❌ Desvantagens
- **Vulnerável ao método Kasiski**
- **Fraca com chaves curtas**
- **Padrões repetitivos revelam informações**
- **Não resiste à criptoanálise moderna**

---

## 🔍 Criptoanálise

### Método de Kasiski (1863)

#### Passo 1: Encontrar Repetições
```
Texto cifrado: LXFOPVEFRHRXFOP...
                ^^^     ^^^
               Repetição encontrada!
```

#### Passo 2: Calcular Distâncias
- Distância entre repetições = múltiplo do comprimento da chave
- Usar MCD (Máximo Divisor Comum) para estimar o comprimento

#### Passo 3: Análise de Frequência
- Dividir o texto em grupos baseados no comprimento da chave
- Aplicar análise de frequência em cada grupo
- Cada grupo é uma cifra de César!

### Índice de Coincidência
```
IC = Σ(ni × (ni-1)) / (N × (N-1))
```
- Texto em português: IC ≈ 0.072
- Texto aleatório: IC ≈ 0.038
- Usado para confirmar o comprimento da chave

---

## 🎯 Exercícios Práticos

### Exercício 1: Cifragem Manual
**Cifre a mensagem**: `HELLO`
**Chave**: `KEY`

**Solução**:
```
H + K = R
E + E = I  
L + Y = J
L + K = V
O + E = S
```
**Resultado**: `RIJVS`

### Exercício 2: Decifragem
**Texto cifrado**: `RIJVS`
**Chave**: `KEY`
**Encontre o texto original**

### Exercício 3: Programação
Modifique o código Rust para:
1. Aceitar entrada do usuário
2. Implementar a função de decifragem
3. Tratar caracteres especiais e espaços

### Exercício 4: Criptoanálise
**Texto cifrado**: `QPWKALVRXCQZIKGRBPFAEOMFLJMFVGVXZWQOBALWUQZFBKLKXXRXWQZFBKLKXXRXW`

Tarefas:
1. Encontre repetições no texto
2. Estime o comprimento da chave
3. Tente quebrar a cifra

---

## 🔧 Laboratório Prático

### Configuração do Ambiente
```bash
# Clone o repositório
git clone <seu-repositorio>
cd cifra_vigenere

# Execute o código
cargo run
```

### Melhorias Sugeridas
1. **Interface de usuário**: Adicionar input interativo
2. **Função de decifragem**: Implementar o processo inverso
3. **Tratamento de erros**: Validar entrada
4. **Análise estatística**: Calcular IC e frequências

---

## 🎓 Pontos-Chave para Cyber Segurança

### Por que Estudar Cifras Clássicas?
1. **Fundamentos**: Base para criptografia moderna
2. **Pensamento crítico**: Desenvolve habilidades de análise
3. **História**: Compreender evolução da segurança
4. **Vulnerabilidades**: Identificar padrões de fraqueza

### Aplicações Modernas
- **CTF (Capture The Flag)**: Desafios de criptografia
- **Pentest**: Identificar cifras fracas em sistemas legados
- **Educação**: Ensinar conceitos de segurança
- **Análise forense**: Decifrar comunicações antigas

### Lições para Segurança Atual
1. **Chaves fracas**: Problema ainda atual
2. **Reutilização de chaves**: Vulnerabilidade persistente
3. **Análise de padrões**: Base da criptoanálise moderna
4. **Importância da aleatoriedade**: Princípio fundamental

---

## 📝 Avaliação

### Critérios de Avaliação
- **Compreensão teórica** (30%)
- **Implementação prática** (40%)
- **Análise crítica** (20%)
- **Participação** (10%)

### Entregáveis
1. Código modificado com melhorias
2. Relatório de criptoanálise do exercício 4
3. Comparação com outras cifras estudadas

---

## 📚 Referências

1. **Kahn, David**. "The Codebreakers" - História da criptografia
2. **Singh, Simon**. "The Code Book" - Introdução acessível
3. **Schneier, Bruce**. "Applied Cryptography" - Referência técnica
4. **Stinson, Douglas**. "Cryptography: Theory and Practice"
5. **RFC 4949**: Internet Security Glossary

### Links Úteis
- [Crypto Museum - Vigenère](https://www.cryptomuseum.com/)
- [CryptoClub - Interactive Tools](https://www.cryptoclub.org/)
- [Practical Cryptography](http://practicalcryptography.com/)

---

## 🏆 Desafio Bonus

**Missão**: Implementar um quebrador automático de Vigenère que:
1. Detecte automaticamente o comprimento da chave
2. Use análise de frequência para encontrar a chave
3. Decifragem automática do texto

**Prazo**: Próxima aula
**Recompensa**: Pontos extras na avaliação!

---

*Esta aula foi preparada com base no projeto prático de implementação da Cifra de Vigenère em Rust. O código fonte está disponível no repositório do curso.*

**Professor**: [Seu Nome]  
**Data**: Novembro 2025  
**Disciplina**: Cyber Segurança