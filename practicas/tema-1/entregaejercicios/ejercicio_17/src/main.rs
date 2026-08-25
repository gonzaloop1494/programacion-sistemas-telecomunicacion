fn bin_search (t: i32, a: &[i32]) -> isize {
    let mut left = 0;
    let mut right = (a.len() as isize) -1;
    while left <= right {
        let mid = (left + right) / 2;
        if a[mid as usize] == t {
            return mid as isize;
        } else if t < a[mid as usize] {
            right = mid - 1;
        } else { //t > a[mid]
            left = mid + 1;
        }
    }
    -1
}

fn count_forbidden(whitelist: &mut [i32], attempts: &[i32]) -> usize {
    whitelist.sort(); //ordena los elementos del array de menor a mayor
    let mut total: usize = 0; //contador de número de direcciones de attempts que no se encuentran en whitelist
    for &ip_address in attempts { //Iteramos sobre cada elemento en attempts
        if bin_search(ip_address, whitelist) == -1 { //si no ha sido encontrada esa dirección de attempts en la whitelist
            total += 1;
        }

    }
    total
}

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};

// Función para leer un fichero de enteros almacenados en path, uno por línea
fn read(path: &str) -> Result<Vec<i32>, io::Error> {
    let file = File::open(path)?;
    let br = BufReader::new(file);

    let mut v = Vec::new();
    for line in br.lines() {
        let line = line?;
        let n = line
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(n);
    }

    Ok(v)
}

#[test]
fn small() {
    let mut whitelist = read("whitelist-small.txt").unwrap();
    let attempts = read("attempts-small.txt").unwrap();

    let whitelist = &mut whitelist[..];
    let attempts = &attempts[..];

    assert_eq!(3, count_forbidden(whitelist, attempts));
}


#[test]
fn large() {
    let mut whitelist = read("whitelist-large.txt").unwrap();
    let attempts = read("attempts-large.txt").unwrap();

    let whitelist = &mut whitelist[..];
    let attempts = &attempts[..];

    assert_eq!(367966, count_forbidden(whitelist, attempts));
}

fn main() {}