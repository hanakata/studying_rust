//↓2つのextern crateはCargo.tomlに書いたクレートを使うためのもの
extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

//クレートの公開されている機能を利用するためにuse宣言を書く
use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

//3000番ポートで待ちうけするだけ
fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
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

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    //form_data = match request.get_ref::<UrlEncodedBody>()の結果から判断。ifのようなswitchのような
    //request.get_ref::<UrlEncodedBody>()を読んでリクエストのボディをパース。
    //<UrlEncodedBody>は型パラメータでRequestのどの部分をメソッドget_refで取り出すかを指定。
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };
    //パースされたデータからnというクエリパラメータの値を取り出す。
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        //u64:: from_str(&unparsed)の結果から判断。ifのようなswitchのような
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!(
                    "Value for 'n' parameter not a number: {:?}\n",
                    unparsed
                ));
                return Ok(response);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!(
        "The gratest common divisor of the numbers {:?} is <b>{}</b>\n",
        numbers, d
    ));
    Ok(response)
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
