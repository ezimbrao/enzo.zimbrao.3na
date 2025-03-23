// src/lib.rs

pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    for i in 0..len {
        product *= *ptr.offset(i as isize);
    }
    product
}
// o erro estava no indice que começava do 1 nao do 0 pulando o primeiro elemento do vetor 




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let array = [2, 3, 4]; // Produto: 2*3*4 = 24
        unsafe {
            assert_eq!(multiply_array(array.as_ptr(), array.len()), 24);
        }
    }
}
// imcompatibilidade dos parametros do mutiplay_array, pois estava esperndo um ponteiro bruto, que nao era
//o que estava acontecendo, alem de nao ter um unsafe para manipular os ponteiro brutos 


// a funcao multiply_array nao usa o fator "factor", se tornando inutil no codigo, entao foi retirado