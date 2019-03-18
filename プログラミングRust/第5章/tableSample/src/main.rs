use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;
fn show(table: &Table){
    for(artist, works) in table{
        println!("works by {}:",artist);
        for work in works{
            println!("{}",work);
        }
    }
}


fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),vec!["many madrigals".to_string(),
    "Tenebra Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),vec!["The Musicians".to_string(),
    "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),vec!["Perseus with the head of Medusa".to_string(),
    "a salt cellar".to_string()]);
    // show(table);
    show(&table);
    //showでtableの所有権が関数に移動している場合呼び出しはエラーとなる。
    //エラーメッセージ：value borrowed here after move
    //&tableにすることで共有参照となりエラーはなくなる。
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    sort_works(&mut table);
    println!("");
    show(&table);
}

//値をソートする場合共有参照では変更ができない。
//なので可変参照を利用する
fn sort_works(table: &mut Table){
    for(_artist, works) in table{
        works.sort();
    }
}