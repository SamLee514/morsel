pub fn show() -> Result<(), std::io::Error> {
    println!();
    println!("Use --manual (-m) to enter manual mode and --dit-length=... (-d=...) to set the length of a dit in milliseconds (default 500)");
    println!();
    println!("** Note that dit length is not used in manual mode and should be greater than your machine's key repeat delay.");
    println!("** If you find it to be too slow, you can decrease your machine's key repeat delay or double tap instead of holding.");
    println!();
    println!("Regular mode:");
    println!("- Tap any key to input a dit (.). Hold any key (or double tap within one dit length) to input a dah (_).");
    println!("- Do not input anything for one dah length to translate current input.");
    println!("- Do not input anything for 7 dit lengths to write a space.");
    println!();
    println!("Manual Mode:");
    println!("- Type '.' and '_' manually. Press space once to translate and another time to input spaces.");
    println!();
    println!("Key:");
    println!("a: ._    n: _.    1: .____");
    println!("b: _...  o: ___   2: ..___");
    println!("c: _._.  p: .__.  3: ...__");
    println!("d: _..   q: __._  4: ...._");
    println!("e: .     r: ._.   5: .....");
    println!("f: .._.  s: ...   6: _....");
    println!("g: __.   t: _     7: __...");
    println!("h: ....  u: .._   8: ___..");
    println!("i: ..    v: ..._  0: _____");
    println!("j: .___  w: .__   +: ._._.");
    println!("k: _._   x: _.._  /: _.._.");
    println!("l: ._..  y: _.__  =: _..._");
    println!("m: __    z: __..");
    println!();
    Ok(())
}
