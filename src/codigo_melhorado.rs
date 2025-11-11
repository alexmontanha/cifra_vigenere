use std::io;

/// Programa aprimorado da Cifra de Vigenère para aula de Cyber Segurança
/// Inclui cifragem, decifragem e interface interativa
fn main() {
    println!("🔐 === CIFRA DE VIGENÈRE ===");
    println!("Implementação para Cyber Segurança");
    println!("---------------------------------");
    
    loop {
        exibir_menu();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entrada");
        
        match input.trim() {
            "1" => modo_cifragem(),
            "2" => modo_decifragem(),
            "3" => modo_demo(),
            "4" => modo_analise(),
            "5" => {
                println!("👋 Obrigado por usar o programa!");
                break;
            }
            _ => println!("❌ Opção inválida! Tente novamente."),
        }
        
        println!("\nPressione Enter para continuar...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
}

fn exibir_menu() {
    println!("\n📋 === MENU PRINCIPAL ===");
    println!("1. 🔒 Cifrar mensagem");
    println!("2. 🔓 Decifrar mensagem");
    println!("3. 🎯 Demonstração (ATTACK/LEMON)");
    println!("4. 📊 Análise de frequência");
    println!("5. 🚪 Sair");
    print!("\nEscolha uma opção: ");
}

fn modo_cifragem() {
    println!("\n🔒 === MODO CIFRAGEM ===");
    
    let texto = obter_texto("Digite o texto a ser cifrado: ");
    let chave = obter_chave();
    
    let texto_cifrado = vigenere_encrypt(&texto, &chave);
    
    println!("\n✅ Resultado:");
    println!("Texto original: {}", texto);
    println!("Chave usada:    {}", chave);
    println!("Texto cifrado:  {}", texto_cifrado);
    
    // Mostrar processo passo a passo para fins educacionais
    mostrar_processo_detalhado(&texto, &chave, &texto_cifrado);
}

fn modo_decifragem() {
    println!("\n🔓 === MODO DECIFRAGEM ===");
    
    let texto_cifrado = obter_texto("Digite o texto cifrado: ");
    let chave = obter_chave();
    
    let texto_original = vigenere_decrypt(&texto_cifrado, &chave);
    
    println!("\n✅ Resultado:");
    println!("Texto cifrado:  {}", texto_cifrado);
    println!("Chave usada:    {}", chave);
    println!("Texto original: {}", texto_original);
}

fn modo_demo() {
    println!("\n🎯 === DEMONSTRAÇÃO CLÁSSICA ===");
    println!("Exemplo usado na aula: ATTACK AT DAWN com chave LEMON");
    
    let texto = "ATTACKATDAWN";
    let chave = "LEMON";
    
    let texto_cifrado = vigenere_encrypt(texto, chave);
    let texto_decifrado = vigenere_decrypt(&texto_cifrado, chave);
    
    println!("\n📋 Processo completo:");
    println!("1. Texto original: {}", texto);
    println!("2. Chave:          {}", chave);
    println!("3. Texto cifrado:  {}", texto_cifrado);
    println!("4. Verificação:    {}", texto_decifrado);
    
    // Mostrar tabela de conversão
    println!("\n📊 Tabela de conversão:");
    println!("Pos | Char | Chave | Cifrado | Explicação");
    println!("----|------|-------|---------|------------");
    
    let chave_expandida = expandir_chave(texto, chave);
    for (i, c) in texto.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            let k = chave_expandida.chars().nth(i).unwrap();
            let cifrado = shift_char(c, k);
            let pos_c = (c as u8 - b'A') as usize;
            let pos_k = (k as u8 - b'A') as usize;
            let pos_resultado = (pos_c + pos_k) % 26;
            
            println!(" {} | {} | {} | {} | {}+{}={} mod 26",
                i, c, k, cifrado, pos_c, pos_k, pos_resultado);
        }
    }
}

fn modo_analise() {
    println!("\n📊 === ANÁLISE DE FREQUÊNCIA ===");
    
    let texto = obter_texto("Digite o texto para análise: ");
    let frequencias = calcular_frequencias(&texto);
    
    println!("\n📈 Frequência das letras:");
    println!("Letra | Quantidade | Percentual");
    println!("------|------------|------------");
    
    let total = texto.chars().filter(|c| c.is_ascii_alphabetic()).count();
    
    for letra in 'A'..='Z' {
        let count = frequencias.get(&letra).unwrap_or(&0);
        let percentual = if total > 0 {
            (*count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        
        if *count > 0 {
            println!("  {}   |     {}      | {:5.1}%", letra, count, percentual);
        }
    }
    
    let ic = calcular_indice_coincidencia(&texto);
    println!("\n📊 Índice de Coincidência: {:.4}", ic);
    
    if ic > 0.060 {
        println!("💡 IC alto - provável texto em português ou chave curta");
    } else if ic < 0.045 {
        println!("💡 IC baixo - provável cifra com chave longa ou texto aleatório");
    } else {
        println!("💡 IC intermediário - análise inconclusiva");
    }
}

fn obter_texto(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");
    
    // Limpar e normalizar o texto
    input.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}

fn obter_chave() -> String {
    loop {
        print!("Digite a chave (apenas letras): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entrada");
        
        let chave: String = input.trim()
            .to_uppercase()
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect();
        
        if chave.is_empty() {
            println!("❌ Chave deve conter pelo menos uma letra!");
            continue;
        }
        
        if chave.len() > 50 {
            println!("⚠️  Chave muito longa! Usando apenas os primeiros 50 caracteres.");
            return chave[..50].to_string();
        }
        
        return chave;
    }
}

fn vigenere_encrypt(texto: &str, chave: &str) -> String {
    let chave_expandida = expandir_chave(texto, chave);
    
    texto.chars()
        .zip(chave_expandida.chars())
        .map(|(t, k)| {
            if t.is_ascii_alphabetic() {
                shift_char(t, k)
            } else {
                t
            }
        })
        .collect()
}

fn vigenere_decrypt(texto_cifrado: &str, chave: &str) -> String {
    let chave_expandida = expandir_chave(texto_cifrado, chave);
    
    texto_cifrado.chars()
        .zip(chave_expandida.chars())
        .map(|(c, k)| {
            if c.is_ascii_alphabetic() {
                unshift_char(c, k)
            } else {
                c
            }
        })
        .collect()
}

fn expandir_chave(texto: &str, chave: &str) -> String {
    let mut chave_expandida = String::new();
    let mut chave_iter = chave.chars().cycle();
    
    for c in texto.chars() {
        if c.is_ascii_alphabetic() {
            chave_expandida.push(chave_iter.next().unwrap());
        } else {
            chave_expandida.push(' '); // Placeholder para não-letras
        }
    }
    
    chave_expandida
}

fn shift_char(texto_char: char, chave_char: char) -> char {
    let base = b'A';
    let texto_pos = (texto_char as u8 - base) as usize;
    let chave_pos = (chave_char as u8 - base) as usize;
    let resultado_pos = (texto_pos + chave_pos) % 26;
    
    (base + resultado_pos as u8) as char
}

fn unshift_char(cifrado_char: char, chave_char: char) -> char {
    let base = b'A';
    let cifrado_pos = (cifrado_char as u8 - base) as i32;
    let chave_pos = (chave_char as u8 - base) as i32;
    let resultado_pos = (cifrado_pos - chave_pos + 26) % 26;
    
    (base + resultado_pos as u8) as char
}

fn mostrar_processo_detalhado(texto: &str, chave: &str, texto_cifrado: &str) {
    println!("\n🔍 === PROCESSO DETALHADO ===");
    
    let chave_expandida = expandir_chave(texto, chave);
    
    println!("Posição | Original | Chave | Cifrado | Cálculo");
    println!("--------|----------|-------|---------|--------");
    
    for (i, ((t, k), c)) in texto.chars()
        .zip(chave_expandida.chars())
        .zip(texto_cifrado.chars())
        .enumerate()
    {
        if t.is_ascii_alphabetic() {
            let t_num = (t as u8 - b'A') as usize;
            let k_num = (k as u8 - b'A') as usize;
            let resultado = (t_num + k_num) % 26;
            
            println!("   {}    |    {}     |   {}   |    {}    | {}+{}={} mod 26",
                i, t, k, c, t_num, k_num, resultado);
        }
    }
}

use std::collections::HashMap;

fn calcular_frequencias(texto: &str) -> HashMap<char, usize> {
    let mut frequencias = HashMap::new();
    
    for c in texto.chars() {
        if c.is_ascii_alphabetic() {
            *frequencias.entry(c.to_ascii_uppercase()).or_insert(0) += 1;
        }
    }
    
    frequencias
}

fn calcular_indice_coincidencia(texto: &str) -> f64 {
    let frequencias = calcular_frequencias(texto);
    let n = texto.chars().filter(|c| c.is_ascii_alphabetic()).count();
    
    if n <= 1 {
        return 0.0;
    }
    
    let soma_ni_ni_menos_1: usize = frequencias.values()
        .map(|&ni| ni * (ni - 1))
        .sum();
    
    soma_ni_ni_menos_1 as f64 / (n * (n - 1)) as f64
}

use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cifragem_basica() {
        let resultado = vigenere_encrypt("HELLO", "KEY");
        assert_eq!(resultado, "RIJVS");
    }

    #[test]
    fn test_decifragem_basica() {
        let resultado = vigenere_decrypt("RIJVS", "KEY");
        assert_eq!(resultado, "HELLO");
    }

    #[test]
    fn test_exemplo_aula() {
        let resultado = vigenere_encrypt("ATTACKATDAWN", "LEMON");
        assert_eq!(resultado, "LXFOPVEFRHR");
        
        let decifrado = vigenere_decrypt("LXFOPVEFRHR", "LEMON");
        assert_eq!(decifrado, "ATTACKATDAWN");
    }

    #[test]
    fn test_chave_unica_letra() {
        // Cifra de César com chave "A" (deslocamento 0)
        let resultado = vigenere_encrypt("HELLO", "A");
        assert_eq!(resultado, "HELLO");
    }

    #[test]
    fn test_indice_coincidencia_portugues() {
        let texto_portugues = "ESTAEHUMAMENSAGEMDEPORTUGUESESCRITALONGAMENTEPARATESTEDEINDICEDECOINCIDENCIA";
        let ic = calcular_indice_coincidencia(texto_portugues);
        assert!(ic > 0.050); // Deve ser maior que texto aleatório
    }

    #[test]
    fn test_indice_coincidencia_aleatorio() {
        let texto_aleatorio = "QWERTYUIOPASDFGHJKLZXCVBNMMNBVCXZLKJHGFDSAPOIUYTREWQ";
        let ic = calcular_indice_coincidencia(texto_aleatorio);
        assert!(ic < 0.055); // Deve ser menor que texto em português
    }
}