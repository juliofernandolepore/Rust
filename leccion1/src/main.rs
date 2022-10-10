fn main() {
    println!("hola, mundo!");
    println!("practicando rust a traves de lecciones autodidactas");

    //tipos de datos
    /*string*/
    let variable_cadena = "cadena de caracteres";
    println!("yo soy una: {}", variable_cadena);

    /*float o flotante */
    let mi_flotante = 2.90;
    println!("este es un numero flotante con el valor: {}", mi_flotante);

    /*booleano bool o boolean */
    let esta_casado = true;
    println!("este sujeto se encuentra casado? {}", esta_casado);

    /*tipo char, caracter o char unicode */
    let peque_char = "a";
    println!("este es el caracter {}", peque_char);

    /*tipos de datos NUMEROS, la letra i de enteros ya sean negativos o positivo
    mientras que la letra u solo los positivos, los tipos de datos son: i, u, 8,
    16,32, 64, 128 (bits) */

    let diez = 10; //i32 por defecto
    println!("el valor es de {}", diez);

    let edad: u32 = 39; //u de positivo
    println!("el valor es {}", edad);

    let resultado_resta_negativa: i32 = 10 - 60; //i admite positivos y negativos
    println!(
        "la resta es {} y la edad es {}",
        resultado_resta_negativa, edad
    );

    let tamanio: isize = 15; //depende del sistema operativo
    println!("el isize es de {}", tamanio);
    let tamanio2: usize = 30; //depende del sistema operativo
    println!("el usize es de {}", tamanio2);

    //floats o flotantes

    let float1 = 10.00; // f64 por defecto (float64)
    println!("el valor de float1 es {}", float1);
    let float2: f32 = 7.66; // f32 (que es float32)
    println!("este valor de f32 es{}", float2);
    let float3: f64 = 5000000.66656565; //doble flotante (float64)
    println!("este es un float64: {}", float3);

    //variables: convenciones sobre nombres

    let mut _varible_omitir = 25000;
    println!("la variable omitir vale: {}", _varible_omitir);
    /* el guion bajo hace que sea omitida
    por el sistema, el mut significa que podra ser modificada la variable */
    let mut mutable = 50; // sera ignorada por el guion bajo
    println!("mutable inicialmente vale {}", mutable);
    mutable = 100;
    println!("mutable fue modificadada por {}", mutable);

    /* CONSTANTES: las constante se escriben en mayusculas, snake case y se le
    especifica el tipo de dato obligatoriamente , ej: i32 o f32 , ademas de ello
    estas pueden declararse fuera de la funcion principal main */

    const LIMITE_USUARIOS: i32 = 100; // constante entera 32
    println!("el limite de usuarios hoy es de {}", LIMITE_USUARIOS);
    const PI: f32 = 3.14; // constante flotante 32
    println!("el valor de PI es {}", PI);
}
