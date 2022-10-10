/*FUNCIONES EN RUST */
fn saludar_1(nombre: &str) {
    println!("hola {}", nombre);
}
fn masa_corporal(kilos: f32, altura: f32) {
    println!(
        "tu indice de masa corporal es: {}",
        kilos / altura.powf(2.0)
    );
    /* los argumentos se completan en numeros con decimnal (float) */
}
fn main() {
    saludar_1("fernando");
    masa_corporal(92.200, 1.83);
}
