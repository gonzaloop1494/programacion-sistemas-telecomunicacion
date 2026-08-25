mod utils;


use utils::bin_search;


fn count(a: &mut[i32]) -> u32 {
   let mut total: u32 = 0;

    a.sort();
    let mut index: usize = 0;
    for &n in a.iter() {
        if bin_search(-n, &a[index + 1..]) != -1 {
            total += 1
        }
        index += 1;
    }
    total
}





#[test]
fn test_1(){
   assert_eq!(0, count(&mut[0, 1, 3]));
}

#[test]
fn test_2(){
   assert_eq!(1, count(&mut[0, -1, 1]));
}

#[test]
fn test_3(){
   assert_eq!(2, count(&mut[-1, 2, 1, -2]));
}

#[test]
fn test_4(){
   assert_eq!(3, count(&mut[7, -1, 4, 5, 22, -7, 2, 1, -2]));
}

#[test]
fn test_5(){
   assert_eq!(5, count(&mut[43, 25, 7, -1, 4, 5, -37, 22, -7, 2, 1, -25, 37, -2]));
}

#[test]
fn test_6(){

assert_eq!(1, count(&mut[324110, -442472, 626686,  -157678,  508681,  77867,    892346,   -565040,
                      123414, -77867,  155091,  129801,   287381,  604242,   686904,   -247109,
                      982455, -210707, -922943, -738817,  85168,   7184,     -212857,  63665,
                      261049, -445002, 486913,  372622,   980111,  982505,   -951004,  954267,
                      915640, -547139, -885732, -774826,  -227720, -893313,  795588,   971211,
                      791015, 394414,  164076,  -859625,  537219,  -892930,  -845554
                      ]));

}

fn main() {}