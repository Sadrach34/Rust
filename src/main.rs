fn main() {
    println!("Hello, world!");

    println!(" ");
    let x = 5;
    let y = 10;

    let sum = x + y;
    println!("The sum of {} + {} = {}", x, y, sum);
    
    let mut a = 5;
    println!("el valor de a es {}", a);

    a = 10;
    println!("El valor de a ahora es {}", a);

    //---------------------------------------------------------------------------------------------------------------------
    println!(" ");
    //DIFERENCIAAAA aqui resulta que s2 le regalo su valor y s2 ya no se puede usar
    //entonces s3 es el nuevo dueño del valor de s2.
    {
        let s1 = String::from("Hola");
        println!("{}",s1)
    }

    let s2 = String::from("mundo");

    // let s3 = s2;         // s2 ya no se puede usar ownership
    let s3 = &s2;  // s2 sigue siendo el dueño del valor, s3 es una referencia a s2 borrowing

    println!("{}", s2);
    println!("{}", s3);

    let len = String::from("H");

    let len2 = calcular_longitud(&len); // &len es una referencia a len, no es el dueño del valor
    println!("La longitud de '{}', es de '{}'", len, len2);

    // Regla de los Préstamos: Puedes tener múltiples referencias inmutables (&T) O una sola referencia mutable (&mut T),
    // pero no ambas al mismo tiempo. Esto evita que varios lugares intenten cambiar el mismo dato a la vez, una causa común de errores.

    let mut lot = String::from("H");
    
    let lot2 = calcular_y_modificar(&mut lot);

    println!("La longitud de '{}', es de '{}'", lot, lot2);

    //---------------------------------------------------------------------------------------------------------------------
    println!(" ");
    //Condicionales
    //practicamente son iguales a c#

    let num = 7;

    if num > 7 {
        println!("Si es mayor (if-else)")
    } else {
        println!("no no es mayor (if-else)")
    }

    if num < 7 {
        println!("no no es mayor (else if)")
    } else if num > 7 {
        println!("Si si es mayor (else if)")
    } else {
        println!("es igual")
    }

    //---------------------------------------------------------------------------------------------------------------------
    println!(" ");
    //loops en rust
    //Existen 3 tipos de loops (loop, while y for)
    
    //loop -> Un bucle infinito que puedes detener con break
    let mut contador = 0;
    loop {
        contador += 1;
        println!("ciclo loop, {}", contador);
        if contador == 5 {
            println!("si es igual a 5");
            break
        }
    }

    //while -> Se ejecuta mientras una condición sea verdadera.
    println!(" ");
    println!("while");
    let mut contador2 = 0;

    while contador2 < 5{
        contador2 += 1;
        print!("{}, ",contador2)
    }

    //for -> El más común y seguro. Itera sobre los elementos de una colección.
    println!(" ");
    println!(" ");
    println!("for");

    let array = [1,2,3,4,5,6,7,8,9,0];

    for elemento in array.iter(){ // .iter() crea un iterador que recorre cada elemento del array
        println!("El valor es: {}", elemento)
    }

    println!(" ");

    for numero in 1..4{ // aqui itera desde el 1 hasta el 3, el 4 no se incluye
        println!("{}!", numero)
    }

    //---------------------------------------------------------------------------------------------------------------------
    println!(" ");
    //estructuras de datos

}

fn calcular_y_modificar(s: &mut String) -> usize {
    // *s = String::from("mundo");// ← El * desreferencia s
    s.push_str("ola mundo"); // .push_str modifica el valor al que s apunta
    s.len()
}

fn calcular_longitud(s: &str) -> usize {
    s.len()
}
