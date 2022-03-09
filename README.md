# term-calc

//! # A simple command line calculator
//! Ein einfacher Komandozeilenrechner welcher vier grundlegende Rechenoperationen
//! `Addition`, `Subtraktion`, `Multiplikation` und `Division` zur Verfügung stellt.
//! Intern wird mit [`f64`] gerechnet und bei der Ausgabe auf 8 Stellen hinter dem Kommer gerundet, 
//! somit ist dieses Programm mehr als Spielzeug als ein echter Rechner zu betrachten.
//! Es werden jedoch die Vorrangsregeln von Punkt- und Strichrechnung beachtet sowie 
//! der Vorrang von Klammerausdrücken.
//! 
//! Zahlen können entweder als eine Folge von Ziffern oder als ausgeschriebenes
//! Wort eingegeben werden. Operatoren und Klammern können ebenfalls entweder als Zeichen oder
//! Wort eingegeben werden. Es kann dabei auch beliebig gemischt werden, z.B.:
//! * `"1 + (1 - 5)"` oder 
//! * `"eins plus auf eins minus fünf zu"` oder 
//! * `"eins + auf 1 - fünf)"` oder
//! * `"eintausendfünfhundertdreiundsiebzig mal (-34 plus 3 mal sechshunderteinundvierzig) durch acht"` oder
//! * `"eins plus (minus drei mal 6) durch 2 minus (drei mal 4)"`.
//! 
//! Gültige Zeichen:
//! * `1` bis `0`
//! * `+`
//! * `-`
//! * `*`
//! * `/`
//! * `(`
//! * `)`
//! 
//! Gültige Wörter:
//! * `plus`
//! * `minus`
//! * `mal`
//! * `durch`
//! * `auf`
//! * `zu`
//! * und viele ausgeschriebenen Zahlen z.B.: `einhundert` oder 
//! * `eintausenddreihundertfünf` usw.. 