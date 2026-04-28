fn hanoi(n: u32, origem: char, destino: char, auxiliar: char) {
    if n == 1 {
        println!("Mover disco 1 de {} para {}", origem, destino);
        return;
    }

    hanoi(n - 1, origem, auxiliar, destino);
    println!("Mover disco {} de {} para {}", n, origem, destino);
    hanoi(n - 1, auxiliar, destino, origem);
}

fn main() {
    let discos = 3;
    hanoi(discos, 'A', 'C', 'B');
}
