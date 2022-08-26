//los slices son un tipo especial de referencias
//apuntan a una secuencia contigua de elementos de una coleccion (un subconjunto de la coleccion).
fn main() {
    let mut s = String::from("Hey Rust");
    let word = first_word(&s);
    
    //si intentamos lipiar la cadena o modificar su valor, tendremos un error ya que word es un slice a ese owner.
    //s.clear(); //no compila
    println!("{word}");

    //en esta posicion si compila, debido al NLL, ya que infiere que la refencia word ya no se usa, por lo que puede ser desechada
    //teniendo como costo que word ya no puede ser usada.
    s.clear();

    //println!("{word}"); //no compila, por que word ya fue droppeado.

    //String es el tipo cadena y str es el tipo slice de String
    //Las String literales son inmutables y se almacenan en el binario
    //y su funcionamiento se debe a que son slices, es decir son de tipo str
    //si llamamos la funcion first_word con un literal no compila, por que los tipos &String y &str no son iguales (type mismatch).

    //let word2 = first_word("Rust"); // no compila

    //basta con cambiar la signature de la funcion, para que el parametro sea de tipo &str y no &String
    //de esa manera puede recibir tanto &String como &str.
    // esto gracias a las deref coercions.
    let s2 = String::from("rustaceans");
    let word2 = first_word_final("Rust"); // no compila
    let word3 = first_word_final(&s2); // no compila

    println!("el valor de word2 es {} y el de word3 es {}", word2, word3);

    //los slices str son exclusivos de String, pero existen slices para otros tipos como Array

    let a = [1,2,3,4];
    let slice_of_array = &a[..2]; // el limite superior es exclusivo, el tipo de este slice es &[i32] y el funcionamiento interno del slice es el mismo.
    assert_eq!(slice_of_array,&a[..2]);
   
}

//funcion que retorna la primer serie de caracteres (palabra) en un String, antes de un espacio
//si no hay ningun espacio devuelve el String completo
//devuelve el indice donde se encontro el espacio
//esta funcion devuelve el valor del indice, el cual no tiene referencia
//si el String del que se obtuvo el indice cambia, el valor del indice que devolvimos, no se ve afectado por que no estan relacionados.
fn first_word_wrong(s: &String) -> usize {
    let bytes = s.as_bytes();

    //iter() crea un iterador de cada elemento, mientas que enumerate() devuelve cada elemento como una tuple (index,&val)

    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            //cuando encontramos el espacio devolvemos el indice
            return i;
        }
    }
    //cuando no encontramos espacio, retornamos el tamaño del String
    s.len()
}

//para evitar esos problemas de sincronizacion, surgen los slices
//que son referencias a una porcion de la coleccion
//mientras esas referencias slices sean validas, no se puede cambiar el valor de la coleccion.__rust_force_expr!

//devolveremos una referencia slice, el tipo &str significa String slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // devolvemos un puntero a una parte del String (slice)
            //es equivalente a &[..i]
        }
    }
    //cuando no encontramos espacio, retornamos el tamaño del String
    &s[0..s.len()]
    //es equivalente a &s[..], el equivalente al rango [n..len] es [n..]
}

//este metodo acepta referencias tanto de &String como de &str.
fn first_word_final(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // devolvemos un puntero a una parte del String (slice)
            //es equivalente a &[..i]
        }
    }
    //cuando no encontramos espacio, retornamos el tamaño del String
    &s[0..s.len()]
    //es equivalente a &s[..], el equivalente al rango [n..len] es [n..]
}

