fn search(t: i32, a: &[i32]) -> isize {

    for (index, &item) in a.iter().enumerate() {
        if item == t {
            return index as isize;
        }
    }
    -1
}




fn rec_bin_search(t: i32, a: &[i32]) -> isize {
    rec_bin_search_aux(t, a, 0, (a.len() as isize) - 1)
}


fn rec_bin_search_aux(t: i32, a: &[i32], left: isize, right: isize) -> isize {
    if left <= right {
        let middle = (left + right) / 2;
        if a[middle as usize] == t {
            middle
        } else if t < a[middle as usize] {
            rec_bin_search_aux(t, a, left, middle - 1)
        } else { // t > a[middle]
            rec_bin_search_aux(t, a, middle + 1, right)
        }
    } else {
        -1
    }
}
fn bin_search(t: i32, a: &[i32]) -> isize {
    let mut left: isize = 0;
    let mut right: isize = (a.len() as isize) - 1;
    while left <= right {
        let middle = (left + right) / 2;
        if a[middle as usize] == t {
            return middle as isize;
        } else if t < a[middle as usize] {
            right = middle - 1;
        } else { // t > a[middle]
            left = middle + 1;
        }
    }
    -1
}


fn count_forbidden(whitelist: &mut [i32], attempts: &[i32]) -> usize {
    whitelist.sort();
    let mut total: usize = 0;
    for &ip_address in attempts {
        if rec_bin_search(ip_address, whitelist) == -1 {
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



