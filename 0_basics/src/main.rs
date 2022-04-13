fn main() {
/*
    let arr = [1,2,3];
    slice(arr);
*/

/*
let hello:&'static str = "Hello, world!";
string(hello);
*/

/*
let multi:(i32,&'static str,char) = (1,"hi",'s');

assert_eq!(multi, cortege(multi));
*/



}

/*
/// Test Array


fn array (){
    //Массивы имеют тип [T; N]
    // T - абстрактный тип подставляемый во время инициализации
    // N - длина
    // инициализация пустого массива длиной 20 значениями 0
    let a = [0; 20];


    //инициализация, тип Rust сам определяет
    let a = [1, 2, 3]; // a: [i32; 3]
    let a:[i32; 3] = [1, 2, 3];// явно указать тип и размер


    // заполнение массива длина 20 тип i32
    let str = ['t'; 20]; // a: [char; 20]

    // изменяемый
    let mut arr:[i32;20] = [7; 20]; // a: [i32; 20]
    arr[1] = 9;
    println!("{}",arr[1]);// 9
    println!("Длина массива = {}",arr.len());

    let mut names:[&str;3] = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
    names[1]="Jeka";
    println!("{}",names[1]);
}
*/

/*
/// Пример Среза.
///
/// # Срез
///
/// ```
/// let  a = [0, 1, 2, 3, 4];
///    let  complete = &a[..];
/// ```

fn slice(arg:[i32;3]){
    //это ссылка в другую структуру данных, получают доступ к части массива без копирования
    let  a = [0, 1, 2, 3, 4];
    let  complete = &a[..]; // Срез, содержащий все элементы массива `a`
     let middle = &a[1..4]; // Срез `a`: только элементы 1, 2, и 3

    print!("{}",complete[complete.len()-1]);
    print!("{}",arg[arg.len()-1]);
    // вывод среза, он взят по ссылке
    for x in complete {
        print!("{} ", x);
    }
    // вывод массива
    for x in &arg {
        print!("{} ", x);
    }
}
*/

/*
/// Test type & str
/// params:
///  @string &'static str
fn string(string:&'static str){
    let hello:&'static str = string;

    if ! hello.is_empty() {
        print!("Длина {}\n",hello.len());
        print!("{}",hello );
    }
}
*/

/*
/// Test Cortege
/// params:
///  @c (i32,&str,char)
/// return value:
/// (i32,&str,char)

fn cortege(c:(i32,&str,char))->(i32,&str,char){
   // единичный тип одноэлементный кортеж
     let multi  = (0,);// убрать неоднозначность с (0) кортежем через запятую
    print!("единичный тип одноэлементный кортеж {}\n",multi.0);

    // доступ через индексы
    print!("i32 = {} \n &str = {} \n char = {}\n",c.0,c.1,c.2 );

    // доступ через деконструкцию
    let (x , y, z) = c;

    print!("i32 = {} \n &str = {} \n char = {}\n",x,y,z );


    (x , y, z)
}
*/
