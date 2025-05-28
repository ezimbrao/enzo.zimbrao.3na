// O(1) - Constante
fn acessar_elemento(v: &Vec<i32>, idx: usize) -> i32 {
    v[idx] // Sempre leva o mesmo tempo, independente do tamanho de v
}

// O(n) - Linear
fn soma_vetor(v: &Vec<i32>) -> i32 {
    let mut soma = 0;
    for &num in v.iter() {
        soma += num;
    }
    soma
}

// O(n^2) - Quadrática
fn todas_as_somas(v: &Vec<i32>) -> Vec<i32> {
    let mut resultados = Vec::new();
    for &x in v.iter() {
        for &y in v.iter() {
            resultados.push(x + y);
        }
    }
    resultados
}

// O(log n) - Logarítmica

fn busca_binaria(v: &Vec<i32>, alvo: i32) -> bool {
    let mut esquerda = 0;
    let mut direita = v.len() - 1;

    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;
        if v[meio] == alvo {
            return true;
        } else if v[meio] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }
    false
}

// O(n log n) - Linear Logarítmica

fn merge_sort(v: &mut Vec<i32>) {
    if v.len() <= 1 {
        return;
    }

    let meio = v.len() / 2;
    let mut esquerda = v[0..meio].to_vec();
    let mut direita = v[meio..].to_vec();

    merge_sort(&mut esquerda);
    merge_sort(&mut direita);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] < direita[j] {
            v[k] = esquerda[i];
            i += 1;
        } else {
            v[k] = direita[j];
            j += 1;
        }
        k += 1;
    }

    while i < esquerda.len() {
        v[k] = esquerda[i];
        i += 1;
        k += 1;
    }

    while j < direita.len() {
        v[k] = direita[j];
        j += 1;
        k += 1;
    }
}


// O(2^n) - Exponencial

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// O(n!) - Fatorial

fn permutacoes(v: &Vec<i32>, inicio: usize, fim: usize) {
    if inicio == fim {
        println!("{:?}", v);
    } else {
        for i in inicio..=fim {
            let mut v_clone = v.clone();
            v_clone.swap(inicio, i);
            permutacoes(&v_clone, inicio + 1, fim);
        }
    }
}