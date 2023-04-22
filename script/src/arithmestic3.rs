/// #### リスト2.4
/// #### オーバーフローを起こす計算
#[allow(dead_code)]
pub fn overflow(){
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x + y;
    println!("{} + {} = {}", x, y, result);
}