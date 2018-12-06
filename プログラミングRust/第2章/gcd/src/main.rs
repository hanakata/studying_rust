//トレイトの読み込み
//Writeトレイトは整形した文字列をストリームに書き出すwrite_fmtメソッドを持つ。
use std::io::Write;
//FromStrは文字列を解析してその型に変換するfrom_strメソッドを持つ
use std::str::FromStr;
//mainは返り値がないので->は必要ない。
fn main() {
    //numberを可変ローカル変数で宣言しVec::new()でPythonでいうリスト、JSでいう配列を定義する。
    let mut numbers = Vec::new();
    //std::env::args()はイテレータを返す。これは必要に応じて引数を1つずつ生成し引数がなくなったらそれを教える。
    //std::env::args()で得られるイテレータが最初に生成するのはそのプログラム名なのでスキップするためskipメソッドを呼び出して
    //それをスキップしている。
    for arg in std::env::args().skip(1) {
        //u64::from_strを呼び出してargに格納された引数を符号なし64bit数にパース
        //from_str関数はパースした値ではなく成功したかを返す。Ok(v)だとvは生成した値、Err(e)だと失敗でeは理由
        //パースが成功したかはexpectを利用して確認する。エラーの場合は指定された文字列を表示して処理を中断。
        //今回はnumbers.pushで成功した場合はnumbersに追加。
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        //.unwrap()はエラーメッセージが正しく表示されたかをチェック。失敗した場合はエラー
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // &はベクタの所有権が別にあり2番目以降への参照を行っているだけというときに利用
    // *は参照解決時に利用
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    //標準出力
    println!("The greatest common divisor of {:?} is {}", numbers, d)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
