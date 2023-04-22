/// #### リスト2.3
/// #### 算術演算子の利用と結果の出力
/// #### 引数x, y 計算対象の値
#[allow(dead_code)]
pub fn methods(x: i32, y: i32){
    // 算術演算トレイトの利用宣言をする
    use std::ops::{ Add, Sub, Mul, Div, Rem};
    // メソッドを使って算術演算を実行
    println!("{} + {} = {}",x, y, x.add(y));
    println!("{} - {} = {}",x, y, x.sub(y));
    println!("{} * {} = {}",x, y, x.mul(y));
    println!("{} / {} = {}",x, y, x.div(y));
    println!("{} % {} = {}",x, y, x.rem(y));
}