use rand::prelude::*;
use std::collections::HashSet;

// Voltea los pancakes
fn flip_pancakes(pancakes: &mut [char], index: usize) {
    if pancakes.len() < 2 { return; }
    pancakes[..index].reverse();
}

// Hace los sucesores de una permutación
fn generar_sucesores(pancakes: &Vec<char>) -> Vec<Vec<char>> {
    let mut sucesores = vec![];
    for i in 1..=pancakes.len() {
        let mut sucesor = pancakes.clone();
        flip_pancakes(&mut sucesor, i);
        sucesores.push(sucesor);
    }
    sucesores
}

// Función que genera una cadena aleatoria de caracteres que representen los pancakes a ordenar
fn fill_pancakes(num_pancakes: usize) -> Vec<char> {
    let dict = "abcdefghijklmnopqrstuvwxyz";
    let mut empty_pancakes = Vec::new();
    let mut seen_chars = HashSet::new();
    for _ in 0..num_pancakes {
        let mut random_char = dict.chars().choose(&mut thread_rng()).unwrap();
        while seen_chars.contains(&random_char) {
            random_char = dict.chars().choose(&mut thread_rng()).unwrap();
        }
        seen_chars.insert(random_char);
        empty_pancakes.push(random_char);
    }
    empty_pancakes
}


// Función que implementa la heuristica
fn h4(s: &Vec<char>) -> i32 {
    let mut c = 0;

    if s[0] != 'a' && s[0] != 'b' {
        c = 1;
        for i in 0..s.len()-1 {
            if (s[i] as i32 - s[i+1] as i32).abs() > 1 {
                c += 1;
            }
        }
    }
    c
}

//Funcion que checa si la permutación esta ordenada
fn is_pancake_sorted(pancakes: &Vec<char>) -> bool {
    for i in 1..pancakes.len() {
        if pancakes[i] < pancakes[i - 1] {
            return false;
        }
    }
    true
}

// A*
fn a_star(pancakes: Vec<char>) {
    let mut cola = std::collections::BinaryHeap::new();
    let mut visitados = HashSet::new();
    let mut count = 0;
    cola.push(std::cmp::Reverse((h4(&pancakes), pancakes.clone())));

    visitados.insert(pancakes.clone());
    while let Some(std::cmp::Reverse((_, curr_pancakes))) = cola.pop() {
        count += 1;
        if is_pancake_sorted(&curr_pancakes) {
            println!("Solucion encontrada: {:?}", curr_pancakes);
            println!("Numero de nodos visitados: {}", count);
            return;
        }
        let sucesores = generar_sucesores(&curr_pancakes);
        for sucesor in sucesores {
            if visitados.insert(sucesor.clone()) {
                cola.push(std::cmp::Reverse((h4(&sucesor), sucesor)));
                count += 1;
            }
        }
    }
    println!("No se encontro solucion.");
    println!("Numero de nodos visitados: {}", count);
}

//Funcion principal
fn main() {
    let mut n = String::new();
    println!("Ingrese el numero de caracteres de pancakes: ");
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    let pancakes = fill_pancakes(n);
    println!("Pancakes generados: {:?}", pancakes);
    a_star(pancakes);
}
