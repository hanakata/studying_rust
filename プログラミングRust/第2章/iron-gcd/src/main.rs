//↓2つのextern crateはCargo.tomlに書いたクレートを使うためのもの
extern crate iron;
#[macro_use]
extern crate mime;

//クレートの公開されている機能を利用するためにuse宣言を書く
use iron::prelude::*;
use iron::status;

//3000番ポートで待ちうけするだけ
fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

// 処理すべきリクエストを表すRequest値への参照。&mutは可変参照
// _のついた変数は使わない宣言
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    //set_mutは引数の型でresponseのどの部分に値をセットするかを決める。
    //今回の場合、それぞれresponseの異なる部分に値を設定する。
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    //レスポンステキストはてきすとなのでこういった書き方(r#"～"#)で書くと楽
    response.set_mut(
        r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
    <input type="text" name="n"/>
    <input type="text" name="n"/>
    <button type="submit">Compute GCD</button>
    </form>
    "#,
    );
    Ok(response)
}
