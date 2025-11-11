# 🎯 EXERCÍCIOS PRÁTICOS - CIFRA DE VIGENÈRE
## Lista de Atividades para Laboratório

---

## 📋 EXERCÍCIO 1: CIFRAGEM MANUAL
**Objetivo**: Compreender o processo básico da cifra de Vigenère  
**Tempo**: 15 minutos  
**Nível**: Iniciante  

### Instruções
Cifre as seguintes mensagens usando a tabela de Vigenère:

#### A) Mensagem: `HELLO WORLD`  
**Chave**: `KEY`  
**Resultado esperado**: ________

#### B) Mensagem: `CYBER SECURITY`  
**Chave**: `RUST`  
**Resultado esperado**: ________

#### C) Mensagem: `ATTACK AT DAWN`  
**Chave**: `SECRET`  
**Resultado esperado**: ________

### Tabela de Vigenère (para consulta)
```
    A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
A   A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
B   B C D E F G H I J K L M N O P Q R S T U V W X Y Z A
C   C D E F G H I J K L M N O P Q R S T U V W X Y Z A B
D   D E F G H I J K L M N O P Q R S T U V W X Y Z A B C
E   E F G H I J K L M N O P Q R S T U V W X Y Z A B C D
... (continua)
```

---

## 🔍 EXERCÍCIO 2: DECIFRAGEM MANUAL
**Objetivo**: Aplicar o processo inverso  
**Tempo**: 20 minutos  
**Nível**: Iniciante/Intermediário  

### Instruções
Decifre as seguintes mensagens (dica: subtraia em vez de somar):

#### A) Texto cifrado: `RIJVS`  
**Chave**: `KEY`  
**Texto original**: ________

#### B) Texto cifrado: `LXFOPVEFRHR`  
**Chave**: `LEMON`  
**Texto original**: ________

#### C) Texto cifrado: `HIMGKZPXOQMKZ`  
**Chave**: `VIGENERE`  
**Texto original**: ________

### Fórmula para Decifragem
```
P[i] = (C[i] - K[i mod len(K)] + 26) mod 26
```

---

## 💻 EXERCÍCIO 3: MELHORANDO O CÓDIGO RUST
**Objetivo**: Expandir funcionalidades do programa  
**Tempo**: 45 minutos  
**Nível**: Intermediário  

### Tarefa Principal
Modifique o código original para incluir:

#### 3.1) Função de Decifragem
```rust
fn vigenere_decrypt(ciphertext: &str, key: &str) -> String {
    // Implemente aqui
    todo!()
}
```

#### 3.2) Interface Interativa
```rust
use std::io;

fn main() {
    println!("=== CIFRA DE VIGENÈRE ===");
    println!("1. Cifrar");
    println!("2. Decifrar");
    println!("Escolha uma opção: ");
    
    // Complete a implementação
}
```

#### 3.3) Tratamento de Caracteres Especiais
- Ignorar espaços, pontuação e números
- Converter automaticamente para maiúsculas
- Preservar formatação original

#### 3.4) Validação de Entrada
- Verificar se a chave contém apenas letras
- Verificar se o texto não está vazio
- Tratar erros graciosamente

### Código Base Expandido
```rust
use std::io;

fn main() {
    loop {
        println!("\n=== CIFRA DE VIGENÈRE ===");
        println!("1. Cifrar mensagem");
        println!("2. Decifrar mensagem");
        println!("3. Sair");
        print!("Escolha: ");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erro ao ler entrada");
        
        match choice.trim() {
            "1" => encrypt_mode(),
            "2" => decrypt_mode(),
            "3" => break,
            _ => println!("Opção inválida!"),
        }
    }
}

fn encrypt_mode() {
    // Implemente aqui
}

fn decrypt_mode() {
    // Implemente aqui
}

// Suas funções existentes aqui...
```

---

## 🕵️ EXERCÍCIO 4: CRIPTOANÁLISE - MÉTODO KASISKI
**Objetivo**: Quebrar uma cifra Vigenère sem conhecer a chave  
**Tempo**: 60 minutos  
**Nível**: Avançado  

### Texto Cifrado para Análise
```
QPWKALVRXCQZIKGRBPFAEOMFLJMFVGVXZWQOBALWUQZFBKLKXXRXWQZFBKLKXX
RXWQZFBKLKXXRXWQZFBKLKXXRXWQZFBKLKXXRXWQZFBKLKXXRXWQZFBKLKXX
```

### Etapas da Análise

#### 4.1) Identificar Repetições
**Instrução**: Procure por sequências de 3+ caracteres que se repetem

| Sequência | Posições | Distância |
|-----------|----------|-----------|
| ??? | ?, ? | ? |
| ??? | ?, ? | ? |
| ??? | ?, ? | ? |

#### 4.2) Calcular Possível Comprimento da Chave
**Instrução**: Use o MCD (Máximo Divisor Comum) das distâncias encontradas

Distâncias encontradas: _______________
MCD: _______________
Comprimento provável da chave: _______________

#### 4.3) Análise de Frequência
**Instrução**: Divida o texto em grupos baseados no comprimento da chave

**Grupo 1** (posições 0, k, 2k, ...): ________________
**Grupo 2** (posições 1, k+1, 2k+1, ...): ________________
**Grupo 3** (posições 2, k+2, 2k+2, ...): ________________

#### 4.4) Encontrar a Chave
Para cada grupo, encontre a letra mais frequente e calcule o deslocamento em relação a 'E' (letra mais comum em português).

**Chave descoberta**: _______________

#### 4.5) Verificação
Use a chave encontrada para decifrar o texto completo.

**Texto original**: _______________

---

## 📊 EXERCÍCIO 5: ANÁLISE ESTATÍSTICA
**Objetivo**: Usar matemática para validar quebra de cifra  
**Tempo**: 30 minutos  
**Nível**: Avançado  

### 5.1) Calcular Índice de Coincidência
```
IC = Σ(ni × (ni-1)) / (N × (N-1))
```

Onde:
- `ni` = frequência da letra i
- `N` = total de caracteres

#### Para o texto cifrado original:
```
A: __ B: __ C: __ D: __ E: __ F: __ G: __ H: __ I: __ J: __ K: __ L: __ M: __
N: __ O: __ P: __ Q: __ R: __ S: __ T: __ U: __ V: __ W: __ X: __ Y: __ Z: __
```

**IC calculado**: _______________

#### Para o texto decifrado:
**IC calculado**: _______________

### 5.2) Interpretação
- IC ≈ 0.038: Texto aleatório (cifra não quebrada)
- IC ≈ 0.072: Texto em português (cifra quebrada com sucesso)

**Conclusão**: _______________

---

## 🏆 EXERCÍCIO 6: DESAFIO AVANÇADO
**Objetivo**: Implementar quebrador automático  
**Tempo**: 90+ minutos  
**Nível**: Expert  

### Especificações do Programa
Crie um programa que:

1. **Receba** um texto cifrado como entrada
2. **Detecte automaticamente** o comprimento da chave (1-20)
3. **Use análise de frequência** para encontrar a chave
4. **Retorne** o texto decifrado e a chave encontrada

### Pseudocódigo Base
```python
def quebrar_vigenere(texto_cifrado):
    melhor_ic = 0
    melhor_comprimento = 1
    
    # Testar comprimentos de 1 a 20
    for comprimento in range(1, 21):
        ic = calcular_ic_para_comprimento(texto_cifrado, comprimento)
        if ic > melhor_ic:
            melhor_ic = ic
            melhor_comprimento = comprimento
    
    # Encontrar a chave usando análise de frequência
    chave = encontrar_chave(texto_cifrado, melhor_comprimento)
    
    # Decifrar o texto
    texto_original = decifrar(texto_cifrado, chave)
    
    return texto_original, chave
```

### Implementação em Rust
```rust
fn quebrar_vigenere(ciphertext: &str) -> (String, String) {
    let mut melhor_ic = 0.0;
    let mut melhor_comprimento = 1;
    
    // Implementar detecção de comprimento
    for comprimento in 1..=20 {
        let ic = calcular_ic_para_comprimento(ciphertext, comprimento);
        if ic > melhor_ic {
            melhor_ic = ic;
            melhor_comprimento = comprimento;
        }
    }
    
    // Implementar descoberta da chave
    let chave = encontrar_chave(ciphertext, melhor_comprimento);
    
    // Decifrar
    let texto_original = vigenere_decrypt(ciphertext, &chave);
    
    (texto_original, chave)
}

fn calcular_ic_para_comprimento(text: &str, key_length: usize) -> f64 {
    // Implementar cálculo de IC
    todo!()
}

fn encontrar_chave(text: &str, key_length: usize) -> String {
    // Implementar análise de frequência
    todo!()
}
```

---

## 🧪 EXERCÍCIO 7: LABORATÓRIO DE VULNERABILIDADES
**Objetivo**: Explorar fraquezas da cifra  
**Tempo**: 45 minutos  
**Nível**: Intermediário  

### 7.1) Teste de Chaves Fracas
Teste a resistência da cifra com diferentes chaves:

#### Chaves para testar:
- `A` (chave de 1 letra)
- `AA` (chave repetitiva)
- `ABC` (chave curta)
- `ABCDEFGHIJKLMNOPQRSTUVWXYZ` (chave muito longa)
- `SEGURANCA` (palavra comum)

#### Mensagem padrão: `"ESTA MENSAGEM E CONFIDENCIAL E DEVE SER PROTEGIDA"`

#### Análise:
1. Qual chave é mais fácil de quebrar?
2. Por que chaves longas não são sempre melhores?
3. Qual é o comprimento ideal para uma chave?

### 7.2) Ataque de Dicionário
Implemente um ataque que testa palavras comuns como chave:

```rust
fn ataque_dicionario(ciphertext: &str, dicionario: &[&str]) -> Option<(String, String)> {
    for palavra in dicionario {
        let tentativa = vigenere_decrypt(ciphertext, palavra);
        if parece_portugues(&tentativa) {
            return Some((tentativa, palavra.to_string()));
        }
    }
    None
}

fn parece_portugues(text: &str) -> bool {
    // Implementar heurística simples
    // Ex: verificar se contém vogais suficientes
    todo!()
}
```

---

## 🎯 EXERCÍCIO 8: COMPARAÇÃO DE CIFRAS
**Objetivo**: Compreender evolução da criptografia  
**Tempo**: 30 minutos  
**Nível**: Intermediário  

### Implemente e Compare:

#### 8.1) Cifra de César
```rust
fn cesar(text: &str, shift: u8) -> String {
    // Implementar
    todo!()
}
```

#### 8.2) Cifra de Vigenère (já implementada)

#### 8.3) Cifra de Vernam (One-Time Pad)
```rust
fn vernam(text: &str, key: &str) -> String {
    // Chave deve ter mesmo tamanho do texto
    // Implementar
    todo!()
}
```

### Análise Comparativa:
Complete a tabela:

| Cifra | Segurança | Facilidade | Chave | Resistência |
|-------|-----------|------------|-------|-------------|
| César | _____ | _____ | _____ | _____ |
| Vigenère | _____ | _____ | _____ | _____ |
| Vernam | _____ | _____ | _____ | _____ |

---

## 📝 GABARITO E SOLUÇÕES

### Exercício 1 - Gabarito:
- A) `RIJVS ASVNH`
- B) `TJMGL KUCXVBPJ`  
- C) `SMVIKK MX HMEX`

### Exercício 2 - Gabarito:
- A) `HELLO`
- B) `ATTACKATDAWN`
- C) `CYBERSECURITY`

### Dicas para Exercícios Avançados:
- **Exercício 4**: A chave é `CRYPTO` (6 letras)
- **Exercício 5**: IC do texto decifrado deve ser ≈ 0.072
- **Exercício 6**: Use frequência esperada: E(12%), A(11%), O(10%)...

---

## 🎓 CRITÉRIOS DE AVALIAÇÃO

### Pontuação:
- **Exercícios 1-2**: 2 pontos cada (total: 4 pontos)
- **Exercício 3**: 3 pontos
- **Exercício 4**: 4 pontos
- **Exercícios 5-8**: 2 pontos cada (total: 8 pontos)
- **Participação e discussão**: 1 ponto

### **Total**: 20 pontos

### Critérios Qualitativos:
- ✅ **Compreensão conceitual** (40%)
- ✅ **Implementação correta** (35%)
- ✅ **Análise crítica** (15%)
- ✅ **Criatividade/melhorias** (10%)

---

*Estes exercícios foram desenvolvidos para complementar a aula teórica e proporcionar experiência prática com a Cifra de Vigenère, desde conceitos básicos até técnicas avançadas de criptoanálise.*