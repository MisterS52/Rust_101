fn main() {
    // Declaración de un entero de 32 bits con tipo explícito
    let entero: i32 = 42;
    let numero: i32 = 10;
    // Declaración de un número de punto flotante de 64 bits
    let flotante: f64 = 3.14;
    let decimal: f64 = 2.5;
    // Declaración de un valor booleano
    let booleano: bool = true;
    let resultado = numero as f64 + decimal;
    // Declaración de un carácter Unicode
    let caracter: char = 'a';

    // Declaración de una tupla con tipos diferentes
    let tupla: (i32, f64, char) = (42, 3.14, 'a');

    // Declaración de un arreglo de enteros de tamaño fijo (3 elementos)
    let array: [i32; 3] = [1, 2, 3];

    // --------------------------
    // Aquí mostramos que Rust es fuertemente tipado
    // --------------------------

    // ❌ Esto daría error de compilación porque Rust no convierte automáticamente entre tipos
    // let suma = entero + flotante; 
    // En C sí dejaría, pero en Rust hay que ser explícito

    // ✅ Forma correcta: convertir manualmente el entero a flotante
    let suma = entero as f64 + flotante;
    println!("Suma correcta (con conversión): {}", suma);

    // ❌ Esto daría error porque no se puede usar un booleano como número
    // let resultado = entero + booleano; 

    // ✅ Si realmente queremos usarlo, tenemos que hacer la conversión explícita
    let booleano_como_num = if booleano { 1 } else { 0 };
    let resultado = entero + booleano_como_num;
    println!("Resultado con booleano convertido a número: {}", resultado);

    // ✅ Acceso seguro a un elemento de un arreglo
    println!("El primer elemento del arreglo es: {}", array[0]);
    println!("Hola, Rust en Kali!");
    println!("Número entero: {}", numero);
    println!("Número decimal: {}", decimal);
    println!("Suma (conversión explícita): {}", resultado);
    // ❌ Si intentamos acceder fuera del rango, Rust detecta el error en tiempo de ejecución
    // println!("{}", array[10]); // Esto produce panic y protege la memoria
}

